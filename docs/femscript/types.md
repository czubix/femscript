# Data Types

In Femscript, there are several built-in data types that allow you to store and manipulate various kinds of data. Each type is designed for a specific purpose, such as representing numbers, text, collections, or custom objects.

---

## Overview of Data Types

| Type        | Description                                                     | Example                       |
| ----------- | --------------------------------------------------------------- | ----------------------------- |
| **Str**     | A sequence of characters representing text.                     | `"Hello, world!"`             |
| **Int**     | An integer or floating-point number.                            | `69` \| `4.2`                 |
| **Bool**    | A boolean value representing truth or falsehood.                | `true` \| `false`             |
| **None**    | Represents the absence of a value or a `NoneType`.              | `none`                        |
| **List**    | A collection of elements, which can include mixed types.        | `["example", 69, [1, 2, 3]]`  |
| **Scope**   | A container that encapsulates variables within a defined scope. | `{ x = 10; y = 20; z = 30; }` |
| **PyObject**| A Python object, such as an instance of a class or a callable.  |                               |
| **Error**   | Represents an error state, used to convey issues or exceptions. | `Error("description")`        |

---

## Descriptions And Examples

### Str

**Str** is a type used to represent text, enclosed in double quotes.

Example:

```femscript linenums="1"
greeting = "Hello, world!";
print(greeting); # Outputs: Hello, World!
```

---

### Int

**Int** represents both integer and floating-point numbers.

Example:

```femscript linenums="1"
x = 69;
y = 4.2;
print(x + y); # Outputs: 73.2
```

---

### Bool

**Bool** is a boolean type, representing either `true` or `false`.

Example:

```femscript linenums="1"
is_adult = true;
is_teenager = false;
print(is_adult and !is_teenager); # Outputs: true
```

---

### None

**None** represents the absence of a value. It is useful when you need to signify an "empty" or "unset" state.

Example:

```femscript linenums="1"
value = none;
print(value); # Outputs: none
```

---

### List

**List** is a collection of elements. It can store mixed types, including other lists.

Example:

```femscript linenums="1"
items = [1, "text", [2, 3]];
print(items.0); # Outputs: 1
print(items.1); # Outputs: text
print(items.2); # Outputs: [2, 3]
```

---

### Scope

**Scope** acts as a container for variables and logic. It can be used to group related data.

Example:

```femscript linenums="1"
student = {
    name = "Joe";
    age = 17;
    grades = [4, 3, 5];
};

print(student.name); # Outputs: Joe
print(student.grades.2); # Outputs: 5
```

---

### PyObject

**PyObject** allows femscript to interact with Python objects. These can be instances, callables, or any other Python-defined entities.

```python title="Python binding example" linenums="1"
class Person:
    def __init__(self, name: str, age: int) -> None:
        self.name = name
        self.age = age

femscript.wrap_function(Person)# (1)!
```

1. You can find more about binding python objects [here](/python/#method-wrap_function)

```femscript linenums="1"
person = Person("Joe", 17);
print(type(person)); # Outputs: PyObject
```

---

### Error

**Error* is used to represent issues or exceptions. This type can be returned to indicate a problem during script execution.

Example:

```femscript linenums="1"
person = {
    name = "Joe";
    age = 17;
};

if { person.age < 18 } {
    Error(format("{} is too young", person.name)); # Outputs (in python): Error: Joe is too young
    # Execution stops here, because Joe is too young
}

print("freaky things");
```

## Notes

- **Dynamic typing**: Femscript is dynamically typed, meaning you do not need to declare types explicitly.
- **Type Inference**: The interpreter determines the type based on the value assigned.
- **Python Interoperability**: The **PyObject** type allows seamless integration with Python, enabling advanced scripting capabilities.