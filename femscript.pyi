from typing import TypedDict, Callable, Optional, Any

class Token(TypedDict):
    type: str
    value: str
    number: float
    list: list[Token]
    bytes: bytes
    scope: Optional[list[Variable]]
    pyobject: Optional[object]

class AST(TypedDict):
    type: str
    token: Token
    children: list[AST]

class Variable(TypedDict):
    name: str
    value: Token

class Function(TypedDict):
    name: str
    func: Callable[[list[Token], list[Variable]], None]

def generate_tokens(code: str) -> list[Token]:
    pass

def generate_ast(tokens: list[Token]) -> list[AST]:
    pass

def execute_ast(ast: list[AST], variables: list[Variable], functions: list[Function], modules: dict[str, list[AST]], debug: bool) -> Token:
    pass

def parse_equation(tokens: list[Token]) -> list[Token]:
    pass

def format_string(*args: Any, **kwargs: Any) -> str:
    pass