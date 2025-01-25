# Functions

A **function** in Femscript is a reusable block of code defined with the `fn` keyword. Functions allow you to encapsulate logic and improve code organization. They can accept parameters, perform operations, and return results. In Femscript, the value of the last expression in a function is automatically returned.

---

## Defining Functions

Functions are defined with the `fn` keyword, followed by the function name, a list of parameters in parentheses, and the function body enclosed in curly braces `{}`. For example:

```femscript linenums="1"
fn add(a, b) {
    a + b
}
```

In this example, the function `add` takes two parameters (`a` and `b`) and returns their sum.

---

## Calling Functions

To call a function, use its name followed by arguments in parentheses:

```femscript linenums="1"
print(add(35, 34)); # Outputs: 69
```

---

## Functions with Conditional Logic

Functions can include conditional statements to control their behaviour:

```femscript linenums="1"
fn is_even(number) {
    if { number % 2 == 0 } {
        "even"
    } else {
        "odd"
    }
}

print(is_even(2)); # Outputs: even
print(is_even(3)); # Outputs: odd
```

## Recursive Functions

Femscript supports recursion, where a function can call itself. However, Femscript limits the maximum recursion depth to `10`. If this limit is exceeded, the program will throw a `RecursionError`. For example:

```femscript linenums="1"
fn recursion() {
    recursion()
}

recursion(); # Outputs: RecursionError: Maximum recursion depth exceeded
```