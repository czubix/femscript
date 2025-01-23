from setuptools import setup

setup(
    name = "femscript-lexer",
    version = "0.1",
    py_modules = ["femscript_lexer"],
    entry_points = {
        "pygments.lexers": [
            "femscript = femscript_lexer:FemscriptLexer",
        ]
    }
)