# Scopes

A **scope** in Femscript is a block of code enclosed within curly braces `{}`. It serves as a way to group variables, execute code, and structure data. Scopes can be used for code organization or for creating complex data structures, such as objects that store both data and logic.

In Femscript, a scope is treated as a single token that encapsulates all variables and logic defined within it. Access to these variables is provided using dot notation (`.`).

---

## Creating Scopes

Scopes are defined using `{}`. Within them, you can define variables, perform operations, and create nested scopes. For example:

```femscript linenums="1"
student = {
    id = 10;

    grades = [3, 3, 4, 2, 5];
    grades_sum = sum(grades);
    average_grade = grades_sum / len(grades);

    if { average_grade >= 4.5 } {
        scholarship = "$420";
    } else {
        scholarship = "$69";
    }

    person = {
        name = "Joe";
        surname = "Doe";
        sex = "female";
        age = 17;
        is_adult = age >= 18;
    };
};
```

In this example, `student` is a scope containing several variables and a nested scope `person`.

---

## Accessing Values in Scopes

To access variables or nested scopes, you use dot notation:

```femscript linenums="1"
x = { x = 10; y = 20; };
print(x.x); # Outputs: 10
print(x.y); # Outputs: 20
```

For nested scopes, the same dot notation applies:

```femscript linenums="1"
student = {
    person = {
        name = "Joe";
        surname = "Doe";
    };
};

print(student.person.name); # Outputs: Joe
```

---

## Accessing Lists in Scopes

Lists defined inside scopes use index-based dot notation. Indices are zero-based:

```femscript linenums="1"
classes = {
    subjects = ["math", "physics", "chemistry"];
};
print(classes.subjects.0); # Outputs: math
print(classes.subjects.1); # Outputs: physics
```

---

## Borrowing Variables with `borrow()`

The `borrow` function allows you to "borrow" a variable from a parent scope into a nested scope. The borrowed variable can then be used in the nested scope. Borrowing is indicated by the `borrow()` function and the `&` symbol for the borrowed variable.

Example:

```femscript linenums="1"
x = 10;
y = {
    borrow(x);
    z = &x + 2;
};
print(y.z); # Outputs: 12
```

Explanation:

1. `x` is borrowed by the `y` scope.
2. The variable `z` inside the `y` scope uses the borrowed value of `x`.