---
title: "Python API"
---

# Femscript Python API Documentation

The `Femscript` Python API allows seamless integration of Femscript code with Python. This API provides tools for parsing, executing, and interacting with Femscript, enabling you to bind Python functions, convert data between Python and Femscript, and handle Femscript tokens and AST (Abstract Syntax Tree).

---

## Classes and Functions

### `TokenType`

- "Str"
- "Int"
- "Bool"
- "None"
- "List"
- "Bytes"
- "Scope"

### **class** `Token` : `TypedDict`

- **Structure:**
    - `type`: [TokenType](#tokentype)
    - `value`: str
    - `number`: float
    - `list`: [[Token](#class-token-typeddict)...]
    - `bytes`: bytes
    - `scope`: [[Variable](#class-variable-typeddict)...]?
    - `pyobject`: Any?

---

### **class** `Variable` : `TypedDict`

- **Structure:**
    - `name`: str
    - `value`: [Token](#class-token-typeddict)

---

### **function** `var`

Creates a Femscript variable.

```python
def var(
    name: str,
    value: Optional[Any] = None,
    *,
    variables: Optional[list[Variable]] = None
) -> Variable
```

- **Parameters:**
    - `name`: str - Name of the variable.
    - `value`: Any - Python object
    - `variables`: [[Variable](#class-variable-typeddict)]? - Optional scope for nested variables.
- **Returns:** [Variable](#class-variable-typeddict)
- **Example:**
    ```python linenums="1"
    from femscript import Femscript, var

    fs = Femscript("x + y", variables=[var("x", 35), var("y", 34)])
    ```

---

### **class** `Femscript`

The `Femscript` class provides the primary interface for parsing and executing Femscript code. It allows defining variables, registering custom Python functions, and running Femscript scripts asynchronously.

```python
class Femscript:
    def __init__(
        code: Optional[str] = None,
        *,
        variables: Optional[list[Variable]] = None,
        functions: Optional[list[Callable[[str, list[Token], list[Variable]], Token]]] = None
    ) -> None
```

- **Parameters:**
    - `code`: str? - Femscript code to parse during initialization.
    - `variables`: [[Variable](#class-variable-typeddict)]? - Predefined variables to add to the script's scope.
    - `functions`: [(str, [[Token](#class-token-typeddict)...], [[Variable](#class-variable-typeddict)...]) -> [Token](#class-token-typeddict)]? - Custom Python functions to bind to Femscript.
- **Example:**
    ```python linenums="1"
    from femscript import Femscript, var

    fs = Femscript("x + y", variables=[var("x", 35), var("y", 34)])
    ```

---

#### **classmethod** `to_fs`

Converts a Python object into a Femscript token.

```python
@classmethod
def to_fs(obj: Any) -> Token
```

- **Parameters:**
    - `obj`: Any - Python object to convert.
- **Returns:** [Token](#class-token-typeddict)

---

#### **classmethod** `to_py`

Converts a Femscript token into a Python object.

```python
@classmethod
def to_py(token: Token) -> Any
```

- **Parameters:**
    - `token`: Token - Femscript token to convert.
- **Returns:** Any

---

#### **method** `add_variable`

Adds or updates a variable in the script's scope.

```python
def add_variable(self, variable: Variable) -> None
```

- **Parameters:**
    - `variable`: [Variable](#class-variable-typeddict) - A variable to add or update.

---

#### **method** `wrap_function`

Wraps a Python function to make it callable from Femscript.

```python
def wrap_function(
    self,
    func: Optional[Callable[..., object]] = None,
    *,
    func_name: Optional[str] = None,
    with_name: Optional[bool] = False
) -> Callable
```

- **Parameters:**
    - `func`: Callable? - The Python function to wrap.
    - `func_name`: str? - Name to use for the function in Femscript (default: Python function name)
    - `with_name`: bool? - If `True`, the function receives its Femscript name as the first argument.
- **Returns:** Callable
- **Example:**
    ```python linenums="1"
    fs = Femscript()

    @fs.wrap_function()
    def add(a: int, b: int) -> int:
        return a + b

    @fs.wrap_function()
    async def something(**kwargs) -> Any:
        # do something asynchronously
        ...

    fs.wrap_function(print)
    ```

---

#### async method execute

Executes the parsed AST asynchronously.

```python
async def execute(self, *, debug: bool = False) -> Any
```

- **Parameters:**
    - `debug`: bool? - If `True`, the script execution environment includes the `print` and `debug` built-in functions.
- **Returns:** Any
- **Example:**
    ```python linenums="1"
    from femscript import Femscript

    fs = Femscript("x = 10; x")
    result = await fs.execute()

    print(result) # Outputs: 10
    ```

---

## Examples

1. Basic Execution
    ```python linenums="1"
    import asyncio
    from femscript import Femscript, var

    async def main() -> None:
        fs = Femscript("x = 35; x + y", variables=[var("y", 34)])
        result = await fs.execute()
        print(result) # Ouputs: 69

    if __name__ == "__main__":
        asyncio.run(main())
    ```

2. Binding Python Functions
    ```python linenums="1"
    fs = Femscript("add(35, 34)")

    @fs.wrap_function()
    def add(a: int, b: int) -> int:
        return a + b

    result = await fs.execute()

    print(result) # Outputs: 69
    ```