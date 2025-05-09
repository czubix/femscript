/*
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
*/

use crate::{lexer::{Token, TokenType}, parser::{AST, ASTType}, interpreter::Scope};
use std::str::FromStr;
use pyo3::{prelude::*, types::{PyList, PyDict}};

pub fn convert_token(py: Python, token: Token) -> &PyDict {
    let py_token = PyDict::new(py);
    let mut list: Vec<&PyDict> = Vec::new();

    for token in token.list {
        list.push(convert_token(py, token));
    }

    py_token.set_item("type", format!("{:?}", token._type)).unwrap();
    py_token.set_item("value", token.value).unwrap();
    py_token.set_item("number", token.number).unwrap();
    py_token.set_item("list", PyList::new(py, list)).unwrap();
    py_token.set_item("bytes", token.bytes).unwrap();

    if let Some(scope) = token.scope {
        py_token.set_item("scope", walk_scope(py, scope)).unwrap();
    }

    if let Some(pyobject) = token.pyobject {
        py_token.set_item("pyobject", pyobject).unwrap();
    }

    py_token
}

pub fn convert_to_token(py: Python, token: &PyDict) -> Token {
    let mut list: Vec<Token> = Vec::new();

    for token in token.get_item("list").unwrap().extract::<Vec<&PyDict>>().unwrap() {
        list.push(convert_to_token(py, token));
    }

    Token {
        _type: TokenType::from_str(token.get_item("type").unwrap().extract::<String>().unwrap().as_str()).unwrap(),
        value: token.get_item("value").unwrap().extract::<String>().unwrap(),
        number: token.get_item("number").unwrap().extract::<f64>().unwrap(),
        list,
        bytes: token.get_item("bytes").unwrap().extract::<Vec<u8>>().unwrap(),
        scope: if let Some(scope) = token.get_item("scope") {
            if let Ok(scope) = scope.extract::<Vec<&PyDict>>() {
                Some(get_scope(py, scope))
            } else {
                None
            }
        } else {
            None
        },
        pyobject: if let Some(pyobject) = token.get_item("pyobject") {
            Some(pyobject.into())
        } else {
            None
        },
        rustobject: None
    }
}

pub fn convert_ast(py: Python, ast: Vec<AST>) -> Vec<&PyDict> {
    let mut py_ast = Vec::new();

    for node in ast {
        let py_node = PyDict::new(py);

        py_node.set_item("type", format!("{:?}", node._type)).unwrap();
        py_node.set_item("token", convert_token(py, node.token)).unwrap();
        py_node.set_item("children", convert_ast(py, node.children)).unwrap();

        py_ast.push(py_node);
    }

    py_ast
}

pub fn convert_to_ast(py: Python, ast: Vec<&PyDict>) -> Vec<AST> {
    let mut rust_ast = Vec::new();

    for node in ast {
        rust_ast.push(AST {
            _type: ASTType::from_str(node.get_item("type").unwrap().extract::<String>().unwrap().as_str()).unwrap(),
            token: convert_to_token(py, node.get_item("token").unwrap().extract::<&PyDict>().unwrap()),
            children: convert_to_ast(py, node.get_item("children").unwrap().extract::<Vec<&PyDict>>().unwrap())
        });
    }

    rust_ast
}

pub fn to_pyobject(py: Python, token: Token) -> Py<PyAny> {
    match token._type {
        TokenType::Str => token.value.into_py(py),
        TokenType::Int => if token.number.fract() == 0.0 { (token.number as u64).into_py(py) } else { token.number.into_py(py) },
        TokenType::Bool => if token.number == 1.0 { true } else { false }.into_py(py),
        TokenType::List => {
            let mut list = Vec::new();

            for item in token.list {
                list.push(to_pyobject(py, item));
            }

            list.into_py(py)
        },
        TokenType::Bytes => token.bytes.to_object(py),
        TokenType::None => Python::None(py),
        TokenType::Scope => scope_to_pydict(py, token.scope.unwrap()).to_object(py),
        TokenType::PyObject => token.pyobject.unwrap(),
        _ => unreachable!()
    }
}

pub fn to_token(py: Python, pyobject: Py<PyAny>) -> Token {
    if pyobject.is_none(py) {
        return Token::new_none();
    }

    if let Ok(value) = pyobject.extract::<String>(py) {
        Token::new_string(value)
    } else if let Ok(value) = pyobject.extract::<bool>(py) {
        Token::new_bool(if value == true { "true" } else { "false" }.to_string())
    } else if let Ok(number) = pyobject.extract::<f64>(py) {
        Token::new_int(number)
    } else if let Ok(list) = pyobject.extract::<Vec<Py<PyAny>>>(py) {
        Token::new_list(list.into_iter().map(|item| to_token(py, item)).collect())
    } else {
        Token::new_pyobject(pyobject)
    }
}

pub fn get_scope(py: Python, variables: Vec<&PyDict>) -> Scope {
    let mut scope = Scope::new();

    for variable in variables {
        let name = variable.get_item("name").unwrap().extract::<String>().unwrap();
        let value = convert_to_token(py, variable.get_item("value").unwrap().extract::<&PyDict>().unwrap());

        scope.push_variable(&name, value);
    }

    scope
}

pub fn walk_scope<'a>(py: Python<'a>, scope: Scope) -> &'a PyDict {
    let py_scope = PyDict::new(py);

    for variable in scope.variables {
        py_scope.set_item(variable.name, convert_token(py, variable.value)).unwrap();
    }

    py_scope
}

pub fn scope_to_pydict(py: Python, scope: Scope) -> &PyDict {
    let py_scope = PyDict::new(py);

    for variable in scope.variables {
        py_scope.set_item(variable.name, to_pyobject(py, variable.value)).unwrap();
    }

    py_scope
}