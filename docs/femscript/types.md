# Data types

There are a few types of objects

| Type        | Description                                                                                 | Example                                |
| ----------- | ------------------------------------------------------------------------------------------- | -------------------------------------- |
| **Str**     | A sequence of characters representing text.                                                 | `"Hello, world"`                       |
| **Int**     | An integer or floating-point number.                                                        | `69` / `4.2`                           |
| **Bool**    | A boolean value representing truth or falsehood.                                            | `true` / `false`                       |
| **None**    | Represents the absence of a value or a `NoneType`.                                          | `none`                                 |
| **List**    | A collection of elements, which can include mixed types.                                    | `["example", 69, [1, 2, 3]]`           |
| **Scope**   | A container that encapsulates variables within a defined scope.                             | `{ x = 10; y = 20; z = 30; }`          |
| **PyObject**| A Python object, such as an instance of a class or a callable.                              | `Embed()` or any other Python callable |
| **Error**   | Represents an error state, used to convey issues or exceptions.                             | `Error("description")`                 |