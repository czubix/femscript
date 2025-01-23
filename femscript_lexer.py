from pygments.lexer import RegexLexer
from pygments.token import Keyword, Name, String, Number, Operator, Comment, Error, Text

class FemscriptLexer(RegexLexer):
    name = "Femscript"
    aliases = ["femscript"]

    tokens = {
        'root': [
            (r'#.*', Comment),
            (r'-?\b\d+(\.\d+)?', Number),
            (r'\btrue|false\b', Keyword.Constant),
            (r'\bnone\b|;', Text),
            (r'"([^"\\]|\\.)*$', Error),
            (r'"', String, 'string'),
            (r'\b(fn|if|else|for|while|and|or|borrow|return)\b', Keyword),
            (r'\b\w+(?=\()', Name.Function),
            (r'\b\w+ ?(?=\{)', Name.Function),
            (r'[=><!+\-*\\/]', Operator),
        ],
        'string': [
            (r'[^\\"]+', String),
            (r'\\n', String),
            (r'\\.', Error),
            (r'"', String, '#pop'),
        ]
    }