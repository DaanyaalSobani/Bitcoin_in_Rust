import time

SIZE = 10_000_000
RUNS = 10
data = list(range(SIZE))

def time_it(label, fn):
    # warmup
    fn()
    start = time.perf_counter()
    for _ in range(RUNS):
        fn()
    avg = (time.perf_counter() - start) / RUNS
    print(f"{label}: {avg * 1000:.1f}ms avg over {RUNS} runs")

# map() + filter() with lambdas — lazy, but lambda call overhead per element
def with_map_filter():
    return sum(map(lambda x: x * 3, filter(lambda x: x % 2 == 0, data)))

# generator expression — lazy, no intermediate list, tighter than lambdas
def with_generator():
    return sum(x * 3 for x in data if x % 2 == 0)

# for loop — inline condition, no function call overhead
def with_loop():
    total = 0
    for x in data:
        if x % 2 == 0:
            total += x * 3
    return total

print(f"Dataset : {SIZE:,} elements")
print(f"Operation: filter evens -> multiply by 3 -> sum\n")

time_it("map + filter (lambdas)", with_map_filter)
time_it("generator expression  ", with_generator)
time_it("for loop              ", with_loop)

# sanity check
assert with_map_filter() == with_generator() == with_loop()
print("\nResults match ✓")
