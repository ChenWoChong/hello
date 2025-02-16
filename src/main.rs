use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{Error as ioError, ErrorKind, Read};
use std::net::IpAddr;
use thiserror::Error as thisError;

fn main() {
    _ = foo();
    let _custom_err = ioError::new(ErrorKind::Other, "hello");

    if let Err(e) = "127.0.0.1:8080".parse::<IpAddr>() {
        println!("ip parse err {e}");
    }

    let res = read_file();
    match res {
        Ok(contents) => println!("{contents}"),
        Err(e) => println!("{e}"),
    }

    let res = read_file2();
    match res {
        Ok(contents) => {
            println!("{contents}");
        }
        Err(e) => {
            println!("{e}");
        }
    }

    let res = read_file3();
    match res {
        Ok(contents) => {
            println!("{contents}");
        }
        Err(e) => {
            println!("{e}");
        }
    }

    let res = read_file4();
    match res {
        Ok(contents) => {
            println!("{contents}");
        }
        Err(e) => {
            println!("{}", e.0);
        }
    }
}

fn foo() -> Result<String, String> {
    Ok("hahah".to_string())
}

pub fn foo2(num: i32) -> Result<i32, String> {
    if num == 10 {
        Ok(num)
    } else {
        Err("other".to_string())
    }
}

pub trait MyError: std::error::Error + std::fmt::Display {}

#[derive(Debug)]
struct MyErr;

impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "my err {}", self)
    }
}

impl std::error::Error for MyErr {}

pub fn foo3(num: i32) -> Result<i32, Box<dyn std::error::Error>> {
    if num == 10 {
        Ok(num)
    } else {
        let my_err = MyErr;
        Err(Box::new(my_err))
    }
}

fn read_file() -> Result<String, String> {
    println!("--------- read_file1 --------");
    match File::open("/Users/wochong/Documents/code/hello/Cargo.toml")
        .map_err(|e| format!("open file err {}", e))
    {
        Ok(mut file) => {
            let mut contents = String::new();
            match file
                .read_to_string(&mut contents)
                .map_err(|_| "read file err".to_string())
            {
                Ok(_) => Ok(contents),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

fn read_file2() -> Result<String, String> {
    println!("--------- read_file2 --------");
    File::open("/Users/wochong/Documents/code/hello/Cargo.toml")
        .map_err(|e| format!("open file err {e}"))
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| format!("read file err: {e}"))
                .map(|num| {
                    println!("contents len is {num}");
                    contents
                })
        })
}

fn read_file3() -> Result<String, String> {
    println!("--------- read_file3 --------");
    let mut file = File::open("/Users/wochong/Documents/code/hello/Cargo.toml")
        .map_err(|e| format!("open file err: {e}"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("read file err: {e}"))?;
    Ok(contents)
}

pub struct MyString(String);

impl From<std::io::Error> for MyString {
    fn from(e: std::io::Error) -> Self {
        MyString(format!("{e}"))
    }
}

pub fn read_file4() -> Result<String, MyString> {
    println!("--------- read_file4 --------");
    let mut file = File::open("/Users/wochong/Documents/code/hello/Cargo.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug, thisError)]
pub enum DataStoreErr {
    #[error("data is disconnet.")]
    Disconnect(#[from] std::io::Error),
    #[error("read action {0}")]
    Redaction(String),
    #[error("invalid error expected {expected:?}, found {found:?}")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown error")]
    Unknown,
}
