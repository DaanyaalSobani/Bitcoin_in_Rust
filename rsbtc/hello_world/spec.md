# hello_world — Text Transformer CLI

A command-line tool that takes an operation name and a string, applies a transformation, and prints the result.

## Usage

```
hello_world <op> <text>
```

- Exactly two arguments must be provided. If not, print a usage message to stderr and exit with code 1.
- Print the operation and input text before printing the result.
- Print the result on its own line.

## Operations

### `reverse`
Reverse the characters of the input string.

```
Input:  "Hello World"
Output: "dlroW olleH"
```

### `invert`
Swap the case of every character. Uppercase becomes lowercase, lowercase becomes uppercase. Non-letter characters are unchanged.

```
Input:  "Hello World"
Output: "hELLO wORLD"
```

### `uppercase`
Convert the entire string to uppercase.

```
Input:  "Hello World"
Output: "HELLO WORLD"
```

### `no-spaces`
Remove all whitespace characters from the string.

```
Input:  "Hello World"
Output: "HelloWorld"
```

### `leet`
Replace specific characters with numbers according to this table:

| Character | Replacement |
|-----------|-------------|
| a, A      | 4           |
| e, E      | 3           |
| i, I      | 1           |
| o, O      | 0           |
| s, S      | 5           |
| t, T      | 7           |

All other characters are left unchanged.

```
Input:  "Leet Speak"
Output: "L337 5p34k"
```

### `acronym`
Take the first character of each word, join them, and uppercase the result.

```
Input:  "as soon as possible"
Output: "ASAP"
```

## Error handling

- If fewer or more than two arguments are provided, print to stderr:
  ```
  Usage <program_name> <op> <text>
  ```
  and exit with code 1.

- If an unrecognised operation is provided, print to stderr:
  ```
  Invalid operation: <op>
  ```
  and exit with code 1.
