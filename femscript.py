"""
Copyright 2022-2025 czubix

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
"""

import asyncio
import math

from femscript import generate_tokens, generate_ast, execute_ast, parse_equation

from typing import Optional, TypedDict, Callable, Any

class Scope(dict):
    format_depth = 0

    def __str__(self) -> str:
        Scope.format_depth += 1
        space = "    " * Scope.format_depth
        string = "{\n" + "\n".join(f"{space}{key} = {value!r};" for key, value in self.items()) + f"\n{"    " * (Scope.format_depth - 1)}}}"
        Scope.format_depth -= 1

        return string

    __repr__ = __str__

types = {
    "Str": str,
    "Int": float,
    "Bool": bool,
    "None": type(None),
    "List": list,
    "Bytes": bytes,
    "Scope": dict
}

class Token(TypedDict):
    type: str
    value: str
    number: float
    list: list["Token"]
    bytes: bytes
    scope: Optional[list["Variable"]]
    pyobject: Optional[object]

class AST(TypedDict):
    type: str
    token: Token
    children: list["AST"]

class Variable(TypedDict):
    name: str
    value: Token

class Function(TypedDict):
    name: str
    func: Callable[[list[Token], list[Variable]], None]

def var(name: str, value: Optional[object] = None, *, variables: Optional[list[Variable]] = None) -> Variable:
    if variables is not None:
        token = Femscript.to_fs({})

        for variable in variables:
            token["scope"].append(variable)

        return Variable(name=name, value=token)

    return Variable(name=name, value=Femscript.to_fs(value))

class FemscriptException(Exception):
    pass

class Femscript:
    def __init__(self, code: Optional[str] = None, *, variables: Optional[list[Variable]] = None, functions: Optional[list[Callable[[str, list[Token], list[Variable]], Token]]] = None, modules: Optional[dict[str, str]] = None) -> None:
        self.tokens: list[Token]
        self.ast: list[AST]

        self._variables: list[Variable] = variables or []
        self._functions: list[Function] = functions or []
        self._modules: dict[str, list[AST]] = modules or {}

        if code is not None:
            self.parse(code)

    def parse(self, code: Optional[str] = None) -> None:
        self.tokens = generate_tokens(code or "")
        self.ast = generate_ast(self.tokens)

        for module in self._modules:
            self._modules[module] = generate_ast(generate_tokens(self._modules[module]))

    def add_variable(self, variable: Variable) -> None:
        for index, _variable in enumerate(self._variables):
            if _variable["name"] == variable["name"]:
                break
        else:
            return self._variables.append(variable)

        self._variables[index] = variable

    def add_module(self, name: str, code: str) -> None:
        self._modules[name] = generate_ast(generate_tokens(code))

    @property
    def variables(self) -> dict[str, object]:
        return {item["name"]: self.to_py(item["value"]) for item in self._variables}

    @classmethod
    def fs_type(cls, obj: Any) -> str:
        return {**{value: key for key, value in types.items()}, int: "Int"}.get(type(obj), "PyObject")

    @classmethod
    def to_fs(cls, obj: Any) -> Token:
        token = Token(
            type = _type if (_type := cls.fs_type(obj)) else "PyObject",
            value = "",
            number = 0.0,
            list = [],
            bytes = b""
        )

        token[result[0]] = (result := {
            "Str": lambda: ("value", obj),
            "Int": (_int := lambda: ("number", float(obj))),
            "Bool": _int,
            "List": lambda: ("list", [cls.to_fs(obj) for obj in obj]),
            "Bytes": lambda: ("bytes", obj),
            "None": lambda: ("type", "None"),
            "Scope": lambda: ("scope", [{"name": key, "value": cls.to_fs(value)} for key, value in obj.items()])
        }.get(_type, lambda: ("pyobject", obj))())[1]

        return token

    @classmethod
    def to_py(cls, token: Token) -> Any:
        if "Error" in token["type"]:
            return FemscriptException(token["value"])

        return {
            "Str": (_str := lambda: token["value"]),
            "Int": lambda: n if not (n := token["number"]).is_integer() else math.floor(n),
            "Bool": lambda: bool(token["number"]),
            "List": lambda: [cls.to_py(token) for token in token["list"]],
            "Bytes": lambda: bytes(token["bytes"]),
            "None": lambda: None,
            "Scope": lambda: Scope(**{name: cls.to_py(token) for name, token in token.get("scope", {}).items()}),
            "PyObject": lambda: token["pyobject"]
        }.get(token["type"], _str)()

    @classmethod
    def error(cls, error: str) -> Token:
        return Token(
            type = "Error",
            value = "Error: " + error,
            number = 0.0,
            list = []
        )

    def wrap_function(self, func: Optional[Callable[..., object]] = None, *, func_name: Optional[str] = None, with_name: Optional[bool] = False) -> Callable:
        def wrapper(func: Callable[..., object]):
            if asyncio.iscoroutinefunction(func):
                def wrapper(name: str, args: list[Token] | Token, scope: list[Variable]) -> Token:
                    async def wrapper():
                        try:
                            if not isinstance(args, tuple) and args["type"] == "Scope":
                                return self.to_fs(
                                    await func(*((name,) if with_name is True else ()), **self.to_py(args))
                                )

                            return self.to_fs(
                                await func(*((name,) if with_name is True else ()), *(self.to_py(arg) for arg in args))
                            )
                        except FemscriptException as exc:
                            return self.error(str(exc))

                    return self.to_fs(wrapper())
            else:
                def wrapper(name: str, args: list[Token] | Token, scope: list[Variable]) -> Token:
                    try:
                        if not isinstance(args, tuple) and args["type"] == "Scope":
                            return self.to_fs(
                                func(*((name,) if with_name is True else ()), **self.to_py(args))
                            )

                        return self.to_fs(
                            func(*((name,) if with_name is True else ()), *(self.to_py(arg) for arg in args))
                        )
                    except FemscriptException as exc:
                        return self.error(str(exc))

            self._functions.append(
                Function(
                    name = func_name or func.__name__,
                    func = wrapper
                )
            )

            return wrapper

        if func is not None:
            return wrapper(func)

        return wrapper

    async def execute(self, *, debug: Optional[bool] = False) -> Any:
        result, scope = await execute_ast(self.ast, self._variables, self._functions, self._modules, debug)
        self._variables = [{"name": key, "value": value} for key, value in scope.items()]

        return self.to_py(result)