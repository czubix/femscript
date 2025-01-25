# Operators

Femscript provides a variety of operators for performing arithmetic, comparison, assignment, and logical operations. These operators are intuitive and similar to those in other programming languages, making it easy to perform common tasks like mathematical calculations, value comparisons, and variable assignments.

## Arithemtic Operators

Arithemtic operators are used to perform basic mathematical operations:

| Operator | Description       | Example      | Result of x |
| -------- | ----------------- | ------------ | ----------- |
| `+`      | Addition          | `x = 5 + 3`  | `8`         |
| `-`      | Subtraction       | `x = 10 - 7` | `3`         |
| `*`      | Multiplication    | `x = 4 * 2`  | `8`         |
| `/`      | Division          | `x = 8 / 2`  | `4`         |
| `%`      | Modulus           | `x = 7 % 3`  | `1`         |

---

## Comparison Operators

Comparison operators are used to compare two values and return a boolean (`true` or `false`):

| Operator | Description              | Example  | Result |
| -------- | ------------------------ | -------- | ------ |
| `==`     | Equal to                 | `5 == 5` | `true` |
| `!=`     | Not equal to             | `5 != 3` | `true` |
| `>`      | Greater than             | `7 > 3`  | `true` |
| `>=`     | Greater than or equal to | `5 >= 5` | `true` |
| `<`      | Less than                | `3 < 7`  | `true` |
| `<=`     | Less than or equal to    | `3 <= 3` | `true` |

---

## Assignment Operators

Assignment operators allow you to assign values to variables. In addition to the basic assignment (`=`), Femscript provides shorthand operators for combining assignments with arithmetic operations:

| Operator | Description         | Example         | Result of x |
| -------- | ------------------- | --------------- | ----------- |
| `=`      | Assign value        | `x = 10`        | `10`        |
| `+=`     | Add and assign      | `x = 5; x += 3` | `8`         |
| `-=`     | Subtract and assign | `x = 5; x -= 2` | `3`         |
| `*=`     | Multiply and assign | `x = 4; x *= 2` | `8`         |
| `/=`     | Divide and assign   | `x = 8; x /= 2` | `4`         |
| `%=`     | Modulus and assign  | `x = 7; x %= 3` | `1`         |

---

## Logical Operators

Femscript supports logical operators for combining or inverting conditions. These are typically used within `if` statements or other conditional expressions:

| Operator | Description | Example        | Result  |
| -------- | ----------- | -------------- | ------- |
| !        | Logical NOT | !true          | `false` |
| and      | Logical AND | true and false | `false` |
| or       | Logical OR  | true or false  | `true`  |
|          |             | 0 or 1         | `1`     |
|          |             | "" or "meow"   | `meow`  |

---

## Example Usage

Here's an example showcasing multiple operators in action:

```femscript linenums="1"
x = 35;
y = 34;

# Arithmetic operations
total = x + y;
difference = x - y;
product = x * y;
quotient = x / y;
remainder = x % y;

# Comparison
is_equal = x == y; # false
is_greater = x > y; # true

# Logical
condition = is_greater and x == y + 1; # true because x is greater than y and x equals y+1

# Assignment shorthand
x += 3; # x becomes 38
y *= 2; # y becomes 68

print("Arithemtic:");
print(format("  Sum: {total}\n  Difference: {difference}\n  Product: {product}\n  Quotient: {quotient}\n  Remainder: {remainder}"));
print(format("\nComparison:"));
print(format("  Equal: {is_equal}\n  Greater: {is_greater}"));
print(format("\nLogical condition: {condition}"));
print(format("\nx = {x}\ny = {y}"));

# Outputs:
# Arithemtic:
#   Sum: 69
#   Difference: 1
#   Product: 1190
#   Quotient: 1.0294117647058822
#   Remainder: 1
# Comparison:
#   Equal: false
#   Greater: true
# Logical condition: true
# x = 38
# y = 68
```