from typing import TypedDict, List, Callable, Optional

class Token(TypedDict):
    type: str
    value: str
    number: float
    list: List[Token]
    bytes: bytes
    scope: Optional[List[Variable]]
    pyobject: Optional[object]

class AST(TypedDict):
    type: str
    token: Token
    children: List[AST]

class Variable(TypedDict):
    name: str
    value: Token

class Function(TypedDict):
    name: str
    func: Callable[[List[Token], List[Variable]], None]

def generate_tokens(code: str) -> List[Token]:
    pass

def generate_ast(tokens: List[Token]) -> List[AST]:
    pass

def execute_ast(ast: List[AST], variables: List[Variable], functions: List[Function], debug: bool) -> Token:
    pass