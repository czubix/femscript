---
title: Conditionals
---

# Conditional Statements (`if`)

The `if` statement in Femscript is used to execute code based on a condition. It provides a way to handle decision-making logic in your scripts. Unlike in some other languages, the `condition` in Femscript is not just a boolean expression—it is a **block of code**. The value of the condition is determined by the result of the last operation in the block.

An optional `else` block can be used to execute code when the condition evaluates to `false`.

---

## Syntax

Conditionals in Femscript use the following structure:

```femscript linenums="1"
if { condition } {
    # Code to execute if condition is true
} else {
    # Code to execute if condition is false (optional)
}
```

The `condition` is a block enclosed in `{}` that can contain multiple operations. The value of the condition is determined by the last operation inside the block.

---

## Examples

### Basic Example

```femscript linenums="1"
person = {
    name = "Joe";
    age = 17;

    if { age >= 18 } {
        message = format("{name} is an adult.");
    } else {
        message = format("{name} is a minor.");
    }
};

print(person.message); # Outputs: Joe is a minor.
```

In this example:

1. The condition `{ age >= 18 }` evaluates to `false`.
2. The `else` block is executed.

---

### Complex Conditions

Condition can perform multiple operations:

```femscript linenums="1"
x = 35;
y = 34;

if {
    total = x + y;
    total > 10 # This line is returned as the condition's value
} {
    result = format("{total} is greater than 10.");
} else {
    result = format("{total} is 10 or less.");
}

print(total); # Outputs: 69
print(result); # Outputs: 69 is greater than 10.
```

Here:

1. The `total` variable is calculated within the condition block.
2. The result of the block (`total > 10`) determines which branch of the `if` statement is executed.