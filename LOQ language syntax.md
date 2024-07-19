# LOQ Language Syntax

## Overview

LOQ is a simple programming language designed to support basic arithmetic operations, variable declarations, and print statements. This guide provides a detailed overview of the syntax and usage of LOQ.

## Basic Syntax

- **Statements** in LOQ are ended with a semicolon (`;`).
- **Keywords** are case-sensitive and must be written exactly as specified.

### Variable Declaration

Variables can be declared using the `let` keyword followed by the variable name, an assignment operator (`=`), and an expression.

**Syntax**:
```
let variable_name = expression;
```

**Example**:
```
let x = 5;
let y = 10 + 2;
```

### Arithmetic Operations

LOQ supports basic arithmetic operations: addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`).

**Syntax**:
```
expression + expression;
expression - expression;
expression * expression;
expression / expression;
```

**Example**:
```
let sum = 5 + 10;
let product = 4 * 2;
let difference = 15 - 3;
let quotient = 20 / 4;
```

### Print Statements

LOQ provides two print statements: `pn` for printing on the same line and `pnl` for printing with a newline.

**Syntax**:
```
pn expression;
pnl expression;
```

**Example**:
```
pn "Hello, World!";
pnl "This will print with a newline";
pn 10 + 20;
```

### Comments

LOQ does not currently support comments.

## Complete Examples

### Example 1: Basic Variable Declarations and Arithmetic

```loq
let a = 5;
let b = 10;
let c = a + b;
pn c;       // Output: 15
pnl a * b;  // Output: 50 (followed by a newline)
```

### Example 2: Print Statements

```loq
pnl "Hello, LOQ!";
pn 42;
pnl 10 + 20 * 3;
```

## Usage

1. **Running the Interpreter**:

   ```sh
   cargo build
   cargo run
   ```

2. **Entering Commands**:

   ```sh
   LOQ> let x = 5;
   LOQ> pn x;
   LOQ> pnl 10 + 20;
   LOQ> exit
   ```

This guide provides the basic syntax rules and usage examples for the LOQ language, helping you get started with writing and running LOQ programs.
