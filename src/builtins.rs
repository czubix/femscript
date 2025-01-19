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

use crate::lexer::{Token, TokenType, RustObject};
use crate::interpreter::{execute_ast, get_function, check_if_error, Function, Scope};
use crate::utils::convert_to_token;
use image::{ImageBuffer, Rgb, ImageResult, ImageFormat};
use std::io::Cursor;
use rand::Rng;
use pyo3::{prelude::*, types::PyDict};
use pyo3_asyncio;

#[derive(Clone, Debug)]
pub struct Image {
    pub buffer: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl Image {
    pub fn new(width: u32, height: u32, background: [u8; 3]) -> Result<Self, &'static str> {
        if width < 128 || width > 1920 || height < 128 || height > 1920 {
            return Err("width and height must be between 128 and 1920");
        }

        Ok(Self { buffer: ImageBuffer::from_fn(width, height, |_x, _y| { Rgb(background) }) })
    }

    pub fn write_to(&mut self, bytes: &mut Vec<u8>) -> ImageResult<()> {
        self.buffer.write_to(&mut Cursor::new(bytes), ImageFormat::Png)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 3]) {
        if x < self.buffer.width() && y < self.buffer.height() {
            self.buffer.put_pixel(x, y, Rgb(color));
        }
    }

    pub fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: [u8; 3]) {
        let mut x1 = x1 as i32;
        let mut y1 = y1 as i32;
        let x2 = x2 as i32;
        let y2 = y2 as i32;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            if x1 >= 0 && x1 < self.buffer.width() as i32 && y1 >= 0 && y1 < self.buffer.height() as i32 {
                self.buffer.put_pixel(x1 as u32, y1 as u32, Rgb(color));
            }

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = err;

            if e2 > -dx {
                err -= dy;
                x1 += sx;
            }

            if e2 < dy {
                err += dx;
                y1 += sy;
            }
        }
    }
}

macro_rules! check_args {
    ($name:ident, $args:ident) => {
        if $args.len() == 0 {
            return Token::new_error(TokenType::TypeError, format!("{}() takes 1 argument", $name));
        }
    };

    ($name:ident, $args:ident, $count:expr) => {
        if $args.len() != $count {
            return Token::new_error(TokenType::TypeError, format!("{}() takes {} arguments", $name, $count));
        }
    };

    ($name:ident, $args:ident, $min:expr, $max:expr) => {
        if $args.len() < $min || $args.len() > $max {
            return Token::new_error(TokenType::TypeError, format!("{}() takes {} to {} arguments", $name, $min, $max));
        }
    };
}

async fn print(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args);

    println!("{}", match args[0]._type {
        TokenType::Str => args[0].value.to_string(),
        TokenType::Int => args[0].number.to_string(),
        TokenType::Bool => match args[0].number as i32 {
            0 => "false".to_string(),
            1 => "true".to_string(),
            _ => unreachable!()
        },
        _ => args[0].to_string()
    });

    Token::new_none()
}

async fn debug(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 1, 2);

    Token::new_string(
        match args.len() {
            1 => format!("{:?}", args[0]),
            2 => {
                if args[1].number == 0.0 {
                    format!("{:?}", args[0]).to_string()
                } else if args[1].number == 1.0 {
                    format!("{:#?}", args[0]).to_string()
                } else {
                    unreachable!()
                }
            },
            _ => unreachable!()
        }
    )
}

async fn get(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 2);

    if args[0]._type != TokenType::List {
        return Token::new_error(TokenType::TypeError, "get() takes a list as its first argument".to_string());
    }

    if args[1]._type != TokenType::Int {
        return Token::new_error(TokenType::TypeError, "get() takes an int as its second argument".to_string());
    }

    let index = args[1].number as usize;

    if index >= args[0].list.len() {
        return Token::new_error(TokenType::TypeError, format!("list index out of range: {}", index));
    }

    args[0].list[index].to_owned()
}

async fn len(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args);

    match args[0]._type {
        TokenType::List => Token::new_int(args[0].list.len() as f64),
        TokenType::Str => Token::new_int(args[0].value.len() as f64),
        _ => Token::new_error(TokenType::TypeError, "len() takes a list or string as its first argument".to_string())
    }
}

async fn contains(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 2);

    if args[0]._type != TokenType::List {
        return Token::new_error(TokenType::TypeError, "has() takes a list as its first argument".to_string());
    }

    if let TokenType::Str | TokenType::Int | TokenType::Bool = args[1]._type {} else {
        return Token::new_error(TokenType::TypeError, "has() takes string, int or bool as its second argument".to_string());
    }

    for token in &args[0].list {
        if match token._type {
            TokenType::Str => token.value == args[1].value,
            TokenType::Int | TokenType::Bool => token.number == args[1].number,
            _ => false
        } {
            return Token::new_bool("true".to_string())
        }
    }

    Token::new_bool("false".to_string())
}

async fn split(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 2);

    if args[0]._type != TokenType::Str {
        return Token::new_error(TokenType::TypeError, "split() takes a string as its first argument".to_string());
    }

    if args[1]._type != TokenType::Str {
        return Token::new_error(TokenType::TypeError, "split() takes a string as its second argument".to_string());
    }

    Token::new_list(args[0].value.split(&args[1].value).map(|string| Token::new_string(string.to_string())).collect())
}

async fn join(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 2);

    if args[0]._type != TokenType::List {
        return Token::new_error(TokenType::TypeError, "join() takes a list as its first argument".to_string());
    }

    if args[1]._type != TokenType::Str {
        return Token::new_error(TokenType::TypeError, "join() takes a string as its second argument".to_string());
    }

    Token::new_string(args[0].list.to_owned().into_iter().map(|token| token.value).collect::<Vec<String>>().join(&args[1].value))
}

async fn hex(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args);

    if args[0]._type != TokenType::Str {
        return Token::new_error(TokenType::TypeError, "hex() takes a string as its first argument".to_string());
    }

    Token::new_int(i64::from_str_radix(&args[0].value, 16).unwrap_or_default() as f64)
}

async fn rgb(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 3);

    if args[0]._type != TokenType::Int {
        return Token::new_error(TokenType::TypeError, "rgb() takes an int as its first argument".to_string());
    }

    if args[1]._type != TokenType::Int {
        return Token::new_error(TokenType::TypeError, "rgb() takes an int as its second argument".to_string());
    }

    if args[2]._type != TokenType::Int {
        return Token::new_error(TokenType::TypeError, "rgb() takes an int as its third argument".to_string());
    }

    let r = args[0].number as u64;
    let g = args[1].number as u64;
    let b = args[2].number as u64;

    Token::new_int(((r << 16) | (g << 8) | b) as f64)
}

async fn randint(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 2, 3);

    if args[0]._type != TokenType::Int {
        return Token::new_error(TokenType::TypeError, "randint() takes an int as its first argument".to_string());
    }

    if args[1]._type != TokenType::Int {
        return Token::new_error(TokenType::TypeError, "randint() takes an int as its second argument".to_string());
    }

    let mut rng = rand::thread_rng();
    let mut num = rng.gen::<f64>() * (args[1].number - args[0].number) + args[0].number;

    if args.len() == 3 {
        if args[2]._type != TokenType::Bool {
            return Token::new_error(TokenType::TypeError, "randint() takes a bool as its third argument".to_string());
        }

        if args[2].number == 0.0 {
            num = num.floor()
        }
    } else {
        num = num.floor()
    }

    Token::new_int(num)
}

async fn _format(name: String, args: Vec<Token>, scope: &mut Scope) -> Token {
    check_args!(name, args);

    if args[0].value.len() > 256 {
        return Token::new_error(TokenType::Error, "argument is too large".to_string())
    }

    let mut text = args[0].value.chars().peekable();
    let mut formatted_text = String::new();
    let mut index = 1;

    while let Some(c) = text.next() {
        if c == '{' {
            if let Some(&c) = text.peek() {
                if c == '}' {
                    index += 1;
                    if index > args.len() {
                        return Token::new_error(TokenType::IndexError, "not enough arguments".to_string());
                    }
                    formatted_text += &_str("str".to_string(), vec![args[index-1].to_owned()], scope).await.value;
                    if formatted_text.len() > 1024 {
                        return Token::new_error(TokenType::Error, "result is too large".to_string())
                    }
                    text.next();
                } else {
                    let mut name = String::new();
                    while let Some(c) = text.next() {
                        if c == '}' {
                            let mut found = false;
                            for variable in &scope.variables {
                                if variable.name == name {
                                    formatted_text += &_str("str".to_string(), vec![variable.value.to_owned()], scope).await.value;
                                    if formatted_text.len() > 1024 {
                                        return Token::new_error(TokenType::Error, "result is too large".to_string())
                                    }
                                    found = true;
                                    break
                                }
                            }
                            if !found {
                                return Token::new_error(TokenType::Undefined, format!("{} is not defined", name));
                            }
                            break
                        } else {
                            name += &c.to_string();
                        }
                    }
                }
            } else {
                return Token::new_error(TokenType::SyntaxError, "missing }".to_string());
            }
        } else {
            formatted_text += &c.to_string();
            if formatted_text.len() > 1024 {
                return Token::new_error(TokenType::Error, "result is too large".to_string())
            }
        }
    }

    if formatted_text.len() > 1024 {
        return Token::new_error(TokenType::Error, "result is too large".to_string())
    }

    Token::new_string(formatted_text)
}

async fn _type(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args);

    Token::new_string(format!("{:?}", args[0]._type))
}

async fn _str(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args);

    match args[0]._type {
        TokenType::Str => args[0].to_owned(),
        TokenType::Int => Token::new_string(if args[0].number == 0.0 { "0".to_string() } else { args[0].number.to_string() }),
        TokenType::Bool => Token::new_string(if args[0].number == 1.0 { "true "} else { "false" }.to_string()),
        TokenType::List => Token::new_string(format!("{:}", args[0]).to_string()),
        TokenType::None => Token::new_string("none".to_string()),
        _ => Token::new_error(TokenType::Unsupported, format!("type {:?} is not supported", args[0]._type))
    }
}

async fn _int(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args);

    match args[0]._type {
        TokenType::Str => {
            if let Ok(number) = args[0].value.parse::<f64>() {
                return Token::new_int(number)
            }

            Token::new_error(TokenType::TypeError, format!("type {:?} cannot be converted to int", args[0]._type))
        },
        TokenType::Int => args[0].to_owned(),
        TokenType::Bool => Token::new_int(args[0].number),
        _ => Token::new_error(TokenType::Unsupported, format!("type {:?} is not supported", args[0]._type))
    }
}

async fn map(name: String, args: Vec<Token>, scope: &mut Scope) -> Token {
    check_args!(name, args, 2);

    if args[0]._type != TokenType::List {
        return Token::new_error(TokenType::TypeError, "map() takes a list as its first argument".to_string());
    }

    if args[1]._type != TokenType::Str {
        return Token::new_error(TokenType::TypeError, "map() takes a string as its second argument".to_string());
    }

    let mut result_list: Vec<Token> = Vec::new();

    for arg in args[0].list.to_owned() {
        if let Some(function) = get_function(&args[1].value, &mut scope.to_owned()) {
            if function.args.len() != 1 {
                return Token::new_error(TokenType::TypeError, format!("function {} should take 1 argument", function.name));
            }

            if let Some(body) = &function.body {
                let mut function_scope = Scope {
                    variables: scope.variables.to_owned(),
                    functions: scope.functions.to_owned()
                };

                function_scope.push_variable(function.args[0].as_str(), arg);

                let result = execute_ast(body.to_owned(), &mut function_scope, Some(Token::new(TokenType::Func)), 0).await;

                if check_if_error(&result) {
                    return result;
                }

                result_list.push(result);
            }
        }
    }

    Token::new_list(result_list)
}

async fn _await(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args);

    if let None = args[0].pyobject {
        return Token::new_error(TokenType::TypeError, "await() takes coroutine as its first argument".to_string());
    }

    if let Err(error) = Python::with_gil(|py| {
        if args[0].pyobject.to_owned().unwrap().getattr(py, "__class__").unwrap().getattr(py, "__name__").unwrap().extract::<String>(py).unwrap() != "coroutine" {
            return Err(Token::new_error(TokenType::TypeError, "await() takes coroutine as its first argument".to_string()));
        }

        Ok(())
    }) {
        return error;
    };

    let future = Python::with_gil(|py| {
        let coro = args[0].pyobject.to_owned().unwrap();
        let locals = pyo3_asyncio::tokio::get_current_locals(py).unwrap();

        pyo3_asyncio::into_future_with_locals(&locals, coro.as_ref(py)).unwrap()
    });

    match future.await {
        Ok(result) => Python::with_gil(|py| convert_to_token(py, result.extract::<&PyDict>(py).unwrap())),
        Err(error) => return Token::new_error(TokenType::Error, error.to_string())
    }
}

async fn error(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args);

    if args[0]._type != TokenType::Str {
        return Token::new_error(TokenType::TypeError, "Error() takes a string as its first argument".to_string());
    }

    Token::new_error(TokenType::Error, args[0].value.to_owned())
}

async fn _image(name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 2, 3);

    if args[0]._type != TokenType::Int {
        return Token::new_error(TokenType::Error, "Image() takes an int as its first argument".to_string());
    }

    if args[1]._type != TokenType::Int {
        return Token::new_error(TokenType::Error, "Image() takes an int as its second argument".to_string());
    }

    let mut background = [255, 255, 255];

    if args.len() >= 3 {
        if args[2]._type != TokenType::List {
            return Token::new_error(TokenType::Error, "Image() takes a list of ints as its third argument".to_string());
        }

        background = [args[2].list[0].number as u8, args[2].list[1].number as u8, args[2].list[2].number as u8];
    }

    let image = Image::new(args[0].number as u32, args[1].number as u32, background);

    if let Err(error) = image {
        return Token::new_error(TokenType::Error, error.to_string());
    }

    Token::new_rustobject(RustObject::Image(image.unwrap()))
}

async fn image_get_data(object: Token, name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    let RustObject::Image(mut image) = object.rustobject.unwrap() else { unreachable!() };

    let mut bytes = Vec::new();
    image.write_to(&mut bytes).unwrap();

    Token::new_bytes(bytes)
}

async fn image_set_pixel(object: Token, name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 3);

    if args[0]._type != TokenType::Int {
        return Token::new_error(TokenType::Error, "Image.set_pixel() takes an int as its first argument".to_string());
    }

    if args[1]._type != TokenType::Int {
        return Token::new_error(TokenType::Error, "Image.set_pixel() takes an int as its second argument".to_string());
    }

    if args[2]._type != TokenType::List {
        return Token::new_error(TokenType::Error, "Image.set_pixel() takes a list of ints as its third argument".to_string());
    }

    let RustObject::Image(mut image) = object.rustobject.unwrap() else { unreachable!() };

    let x = args[0].number as u32;
    let y = args[1].number as u32;
    let color = [args[2].list[0].number as u8, args[2].list[1].number as u8, args[2].list[2].number as u8];

    image.set_pixel(x, y, color);

    Token::new_rustobject(RustObject::Image(image))
}

async fn image_draw_line(object: Token, name: String, args: Vec<Token>, _scope: &mut Scope) -> Token {
    check_args!(name, args, 5);

    if args[0]._type != TokenType::Int {
        return Token::new_error(TokenType::Error, "Image.draw_line() takes an int as its first argument".to_string());
    }

    if args[1]._type != TokenType::Int {
        return Token::new_error(TokenType::Error, "Image.draw_line() takes an int as its second argument".to_string());
    }

    if args[2]._type != TokenType::Int {
        return Token::new_error(TokenType::Error, "Image.draw_line() takes an int as its third argument".to_string());
    }

    if args[3]._type != TokenType::Int {
        return Token::new_error(TokenType::Error, "Image.draw_line() takes an int as its fourth argument".to_string());
    }

    if args[4]._type != TokenType::List {
        return Token::new_error(TokenType::Error, "Image.draw_line() takes a list of ints as fifth argument".to_string());
    }

    let RustObject::Image(mut image) = object.rustobject.unwrap() else { unreachable!() };

    let x1 = args[0].number as u32;
    let y1 = args[1].number as u32;
    let x2 = args[2].number as u32;
    let y2 = args[3].number as u32;
    let color = [args[4].list[0].number as u8, args[4].list[1].number as u8, args[4].list[2].number as u8];

    image.draw_line(x1, y1, x2, y2, color);

    Token::new_rustobject(RustObject::Image(image))
}

pub fn get_builtins() -> Vec<Function> {
    vec![
        Function::new_builtin("get"),
        Function::new_builtin("len"),
        Function::new_builtin("contains"),
        Function::new_builtin("split"),
        Function::new_builtin("join"),
        Function::new_builtin("hex"),
        Function::new_builtin("rgb"),
        Function::new_builtin("randint"),
        Function::new_builtin("format"),
        Function::new_builtin("type"),
        Function::new_builtin("str"),
        Function::new_builtin("int"),
        Function::new_builtin("map"),
        Function::new_builtin("await"),
        Function::new_builtin("Error"),
        Function::new_builtin("Image")
    ]
}

pub async fn call_builtin(name: String, args: Vec<Token>, scope: &mut Scope) -> Option<Token> {
    macro_rules! wrap {
        ($func:ident) => {
            if stringify!($func) == name {
                return Some($func(name, args, scope).await);
            }
        };

        ($func:ident, $func_name:literal) => {
            if $func_name == name {
                return Some($func(name, args, scope).await);
            }
        };
    }

    wrap!(print);
    wrap!(debug);
    wrap!(get);
    wrap!(len);
    wrap!(contains);
    wrap!(split);
    wrap!(join);
    wrap!(hex);
    wrap!(rgb);
    wrap!(randint);
    wrap!(_format, "format");
    wrap!(_type, "type");
    wrap!(_str, "str");
    wrap!(_int, "int");
    wrap!(map, "map");
    wrap!(_await, "await");
    wrap!(error, "Error");
    wrap!(_image, "Image");

    None
}

fn get_object_name(object: &Token) -> &'static str {
    match object.rustobject.as_ref().unwrap() {
        RustObject::Image(_) => "Image",
        _ => unreachable!()
    }
}

pub async fn call_method(object: Token, name: String, args: Vec<Token>, scope: &mut Scope) -> Option<Token> {
    if let None = object.rustobject {
        return None;
    }

    macro_rules! wrap {
        ($func:ident, $func_name:literal) => {
            if $func_name == format!("{}_{}", get_object_name(&object), name) {
                return Some($func(object, name, args, scope).await);
            }
        };
    }

    wrap!(image_get_data, "Image_get_data");
    wrap!(image_set_pixel, "Image_set_pixel");
    wrap!(image_draw_line, "Image_draw_line");

    None
}