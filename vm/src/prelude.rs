use crate::{env::Table, panic, Constant};
use std::fs;
use std::io::{self, Write};
fn puts(args: &[Constant]) -> Constant {
    match args[0] {
        Constant::Str(ref s) => println!("{}", s),
        ref other => println!("{}", other),
    };
    Constant::Nil
}

fn print(args: &[Constant]) -> Constant {
    match &args[0] {
        Constant::Str(s) => print!("{}", s),
        other => print!("{}", other),
    };
    Constant::Nil
}

fn input(args: &[Constant]) -> Constant {
    match &args[0] {
        Constant::Str(s) => print!("{}", s),
        other => print!("{}", other),
    };

    if io::stdout().flush().is_err() {
        panic!("Error flushing stdout")
    }
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        panic!("Error reading line")
    }
    input.pop();
    Constant::Str(input)
}

fn head(args: &[Constant]) -> Constant {
    match &args[0] {
        Constant::List(xs) => match xs.head() {
            Some(x) => x.clone(),
            None => Constant::Nil,
        },
        other => panic!("head() expected a list, found {}", other),
    }
}

fn tail(args: &[Constant]) -> Constant {
    match &args[0] {
        Constant::List(xs) => Constant::List(xs.tail()),
        other => panic!("tail() expected a list, found {}", other),
    }
}

fn str(args: &[Constant]) -> Constant {
    Constant::Str(format!("{}", &args[0]))
}
fn create_file(args: &[Constant]) -> Constant {
    use Constant::*;
    match &args[0] {
        Str(ref filename) => {
            let _ = fs::File::create(filename);
        }
        other => panic!("file_create() expected str, found {}", other),
    }
    Nil
}
fn write_file(args: &[Constant]) -> Constant {
    use Constant::*;
    match &args[0] {
        Str(ref content) => match &args[1] {
            Str(ref filename) => {
                let _ = fs::write(filename, content);
            }
            other => panic!("file_write()[1] expected str, found {}", other),
        },
        other => panic!("file_write() expected str, found {}", other),
    }
    Nil
}
fn exists_file(args: &[Constant]) -> Constant {
    use Constant::*;
    match &args[0] {
        Str(ref filename) => {
            if fs::File::open(filename).is_ok() {
                return Bool(true);
            }
        }
        other => panic!("file_exists() expected str, found {}", other),
    }
    Bool(false)
}
fn remove_file(args: &[Constant]) -> Constant {
    use Constant::*;
    match &args[0] {
        Str(ref filename) => {
            let _ = fs::remove_file(filename);
        }
        other => panic!("file_remove() expected str, found {}", other),
    }
    Nil
}
fn read_file(args: &[Constant]) -> Constant {
    use Constant::*;
    match &args[0] {
        Str(filename) => match fs::read_to_string(filename) {
            Ok(v) => Str(v),
            Err(_) => Nil,
        },
        other => panic!("file_read() str, found {}", other),
    }
}
fn r#type(args: &[Constant]) -> Constant {
    Constant::Str(
        match &args[0] {
            Constant::List(_) => "list",
            Constant::Str(_) => "str",
            Constant::Num(_) => "num",
            Constant::Bool(_) => "bool",
            Constant::Sym(_) => "symbol",
            Constant::Nil => "nil",
            Constant::Fun { .. } => "fn",
            Constant::PartialFun { .. } => "fn",
            Constant::NativeFun { .. } => "native fn",
        }
        .into(),
    )
}

fn inspect(args: &[Constant]) -> Constant {
    Constant::Str(format!("{:#?}", &args[0]))
}

fn int(args: &[Constant]) -> Constant {
    let str = match &args[0] {
        Constant::Sym(symbol) => symbol.to_str(),
        Constant::Str(ref str) => str,
        other => crate::panic!("Expected a string or a symbol, found {}", other),
    };

    match str.parse::<f64>() {
        Ok(n) => Constant::Num(n),
        Err(_) => Constant::Nil,
    }
}

pub fn prelude() -> Table {
    let mut prelude = Table::new();
    macro_rules! insert_fn {
        ($name: expr, $fn: expr) => {
            insert_fn!($name, $fn, 1)
        };
        ($name: expr, $fn: expr, $arity:expr) => {
            prelude.insert(
                $crate::Symbol::new($name),
                Constant::NativeFun {
                    arity: $arity,
                    fp: |it| $fn(&*it),
                },
            )
        };
    }

    insert_fn!("puts", puts);
    insert_fn!("print", print);
    insert_fn!("input", input);
    insert_fn!("head", head);
    insert_fn!("tail", tail);
    insert_fn!("str", str);
    insert_fn!("type", r#type);
    insert_fn!("inspect", inspect);
    insert_fn!("int", int);
    insert_fn!("file_read", read_file);
    insert_fn!("file_write", write_file, 2);
    insert_fn!("file_remove", remove_file);
    insert_fn!("file_create", create_file);
    insert_fn!("file_exists", exists_file);
    prelude
}
