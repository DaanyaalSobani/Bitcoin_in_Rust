# Closure Factories — Functions That Return Closures

Reference: `lib/src/types.rs` — `Blockchain::cleanup_mempool`
Examples: `practice/closure_factory/src/main.rs`

---

## The problem this pattern solves

Some functions (like `Vec::retain`) take a closure with a fixed signature:

```rust
pub fn retain<F>(&mut self, f: F) where F: FnMut(&T) -> bool
```

The closure gets one argument — the element — and returns a `bool`. There is no
room to pass extra context.

But sometimes the closure needs extra state: a threshold, a timestamp, or a
`Vec` to collect side-effect data into. A closure factory solves this — a function
that takes the extra context upfront and returns a closure that already has it baked in.

---

## Basic shape

```rust
fn make_threshold_checker(threshold: u64) -> impl Fn(u64) -> bool {
    move |value| value >= threshold
}

let is_large = make_threshold_checker(10_000);
println!("{}", is_large(5_000));  // false
println!("{}", is_large(15_000)); // true
```

`impl Fn(u64) -> bool` — the return type says "I'm returning something callable that
takes a u64 and returns a bool". The concrete closure type is hidden.

`move` — forces the closure to take ownership of `threshold` rather than borrow it.
Required here because the closure outlives the function frame — without `move`,
`threshold` would be borrowed from a stack frame that no longer exists.

---

## When the closure needs to mutate captured state

If the closure needs to modify something (e.g. collect expired items while filtering),
capture a `&mut` reference. The return type needs an explicit lifetime so Rust knows
the closure cannot outlive the reference:

```rust
fn make_keep_checker<'a>(
    max_age: u64,
    expired: &'a mut Vec<u32>,
) -> impl FnMut(&(u64, u32)) -> bool + 'a {
    move |(age, id)| {
        if *age > max_age {
            expired.push(*id);
            false
        } else {
            true
        }
    }
}
```

`FnMut` (not `Fn`) — the closure mutates its captured state (`expired`), so it needs
`FnMut`. A `Fn` closure may only read its captures.

`+ 'a` — the returned closure holds a `&'a mut Vec`. The `'a` ties the closure's
lifetime to that reference so Rust can enforce the closure is dropped before the Vec is.

Usage:

```rust
let mut expired_ids: Vec<u32> = vec![];
mempool.retain(make_keep_checker(60, &mut expired_ids));
// after retain, expired_ids contains the removed tx ids
```

This is exactly the pattern used in `Blockchain::cleanup_mempool` in
`lib/src/types.rs`.

---

## `'_` vs named lifetime `'a`

```rust
// '_ works when there is only one reference in the input
fn make_checker(log: &mut Vec<String>) -> impl FnMut(u64) -> bool + '_ { ... }

// named lifetime required when there are multiple references and Rust
// can't tell which one the output lifetime refers to
fn make_checker<'a>(prefix: &'static str, log: &'a mut Vec<String>) -> impl FnMut(u64) -> bool + 'a { ... }
//                  ^^^^^^^^^^^^^^^ &'static is not ambiguous
//                                         ^^^ 'a explicitly tied to log
```

The compiler error when you write `'_` with multiple references:
```
error[E0106]: missing lifetime specifier
  = help: this function's return type contains a borrowed value,
    but the signature does not say whether it is borrowed from `prefix` or `log`
```

---

## Each call produces an independent closure

```rust
let double = make_multiplier(2);
let triple = make_multiplier(3);
```

`double` and `triple` are two separate closures, each owning their own copy of
`factor`. Calling one has no effect on the other. This is different from a shared
mutable reference — each closure is fully self-contained.

---

## Cross-language comparison

### JavaScript — `bind` and factory functions

JavaScript doesn't have Rust's ownership model but the same pattern appears.

**`Function.prototype.bind`** — partially applies a function by fixing `this` and
optionally fixing leading arguments:

```javascript
function isAboveThreshold(threshold, value) {
    return value >= threshold;
}

const isLarge = isAboveThreshold.bind(null, 10_000);
// null = don't rebind `this`
// 10_000 = fix first argument (threshold)

console.log(isLarge(5_000));   // false
console.log(isLarge(15_000));  // true

const numbers = [500, 15_000, 3_000, 25_000];
console.log(numbers.filter(isLarge));  // [15000, 25000]
```

`bind` returns a new function with some arguments pre-filled. It's the closest
JavaScript equivalent to a closure factory for simple cases.

**`Function.prototype.apply`** — calls a function with a given `this` and an array
of arguments. It does NOT return a new function — it calls immediately. It's for
dynamic dispatch, not partial application:

```javascript
isAboveThreshold.apply(null, [10_000, 15_000]);  // true — called immediately
```

So `bind` is the partial application tool; `apply` is the dynamic-call tool.

**Closure factory in JavaScript** — the more general form, equivalent to Rust's:

```javascript
function makeThresholdChecker(threshold) {
    return (value) => value >= threshold;
    //      ^^^^^^^ arrow function closes over `threshold`
}

const isLarge = makeThresholdChecker(10_000);
console.log(isLarge(5_000));  // false
```

JavaScript closures capture by reference to the outer scope, not by copy. In most
cases this doesn't matter, but it can cause surprises in loops (classic `var` bug):

```javascript
// BUG — all closures capture the same `i` variable
for (var i = 0; i < 3; i++) {
    setTimeout(() => console.log(i), 0);  // prints 3, 3, 3
}

// FIX — each iteration gets its own `i` copy with let
for (let i = 0; i < 3; i++) {
    setTimeout(() => console.log(i), 0);  // prints 0, 1, 2
}
```

Rust's `move` is explicit — you decide per-closure whether to capture by reference
or by value. JavaScript's `let` vs `var` is the closest analogue.

**Mutable side-effect equivalent in JavaScript:**

```javascript
function makeCleanupChecker(maxAge, expiredIds) {
    return ([age, id]) => {
        if (age > maxAge) {
            expiredIds.push(id);  // mutate the outer array directly
            return false;
        }
        return true;
    };
}

const expiredIds = [];
const mempool = [[10, 101], [120, 102], [30, 103], [200, 104]];
const cleaned = mempool.filter(makeCleanupChecker(60, expiredIds));

console.log(cleaned);     // [[10, 101], [30, 103]]
console.log(expiredIds);  // [102, 104]
```

JavaScript doesn't need `&mut` or lifetimes — arrays are passed by reference
automatically. The trade-off is that Rust's borrow checker guarantees at compile time
that `expiredIds` can't be accessed invalidly; JavaScript gives you no such guarantee.

### Python — `functools.partial` and closures

```python
from functools import partial

def is_above_threshold(threshold, value):
    return value >= threshold

is_large = partial(is_above_threshold, 10_000)
print(is_large(5_000))    # False
print(is_large(15_000))   # True

numbers = [500, 15_000, 3_000, 25_000]
print(list(filter(is_large, numbers)))  # [15000, 25000]
```

`functools.partial` is the Python equivalent of `bind` — it fixes leading arguments
and returns a new callable.

Python closure factory:

```python
def make_threshold_checker(threshold):
    def checker(value):
        return value >= threshold
    return checker
```

Or with a lambda:

```python
def make_threshold_checker(threshold):
    return lambda value: value >= threshold
```

---

## Summary

| Language | Partial application tool | Returns new callable? | Notes |
|---|---|---|---|
| Rust | closure factory + `move` | yes | explicit ownership via `move`, lifetimes for `&mut` captures |
| JavaScript | `bind` or factory function | yes (`bind`), no (`apply`) | `apply` calls immediately; closures capture by reference |
| Python | `functools.partial` or nested def | yes | no ownership model; captures by reference |

The core idea is identical across all three: take context upfront, return something
callable that already has it baked in, so the caller only needs to supply the
varying argument.
