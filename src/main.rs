fn main() {
    let my_url = "https://www.baidu.com".to_string();
    let _url = Url {
        protocol: &my_url[0..1],
        host: &my_url[2..3],
        path: &my_url[4..5],
        qurey: &my_url[7..9],
        fragment: &my_url[10..11],
    };

    let a = "i have apple, this is real apple.".to_string();
    let sum;
    {
        let b = "i have apple pen".to_string();
        sum = longest(a.as_ref(), b.as_ref());
        println!("longest is {}", sum);
    }
}

#[allow(dead_code)]
struct Url<'a> {
    protocol: &'a str,
    host: &'a str,
    path: &'a str,
    qurey: &'a str,
    fragment: &'a str,
}

impl<'a> Url<'a> {
    fn _play() {}
}

#[allow(dead_code)]
pub struct Request<'a> {
    url: Url<'a>,
    body: String,
}

static AA: &'static str = "aaa";
pub fn foo() -> &'static str {
    AA
}

pub fn foo1(a: &str) -> &str {
    a
}

pub fn foo2<'a, 'b: 'a>(i: i32, a: &'a str, b: &'b str) -> &'a str {
    if i == 10 {
        a
    } else {
        println!("{}", b);
        b
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct A {
    foo: String,
}

impl A {
    pub fn play(&self, a: &str, b: &str) -> &str {
        &self.foo
    }
    pub fn play2<'a>(&self, a: &'a str, b: &str) -> &'a str {
        a
    }
}
