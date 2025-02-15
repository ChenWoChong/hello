use std::fmt;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::net::IpAddr;

fn main() {
    _ = foo();
    let _custom_err = Error::new(ErrorKind::Other, "hello");

    if let Err(e) = "127.0.0.1:8080".parse::<IpAddr>() {
        println!("ip parse err {e}");
    }

    let res = read_file();
    match res {
        Ok(contents) => println!("{contents}"),
        Err(e) => println!("{e}"),
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

fn foo3(num: i32) -> Result<i32, Box<dyn std::error::Error>> {
    if num == 10 {
        Ok(num)
    } else {
        let my_err = MyErr;
        Err(Box::new(my_err))
    }
}

fn read_file() -> Result<String, String> {
    match File::open("/Users/wochong/Documents/code/hello/Cargo.toml").map_err(|e| format!("open file err {}", e)) {
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
