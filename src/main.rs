fn main() {
    let _a = doit0();

    let _a = doit::<Atype>();
    let _b = doit::<Btype>();
    let _c = doit::<Ctype>();

    let _a = doit2();

    let _a = doit3(1);

    doit4(&Atype);
    doit4(&Btype);
    doit4(&Ctype);

    let a = Atype;
    let b = Btype;
    let c = Ctype;
    let _v: Vec<&dyn B> = vec![&a, &b, &c];
}

#[derive(Debug)]
struct Atype;
struct Btype;
struct Ctype;

trait A {
    fn new() -> Self;
}

impl A for Atype {
    fn new() -> Self {
        Atype
    }
}

impl A for Btype {
    fn new() -> Self {
        Btype
    }
}

impl A for Ctype {
    fn new() -> Self {
        Ctype
    }
}

fn doit0() -> Box<dyn std::fmt::Debug> {
    let a = Atype;
    Box::new(a)
}

fn doit<T: A>() -> T {
    T::new()
}

fn doit2() -> impl A {
    Atype
}

trait B {}
impl B for Atype {}
impl B for Btype {}
impl B for Ctype {}

fn doit3(n: i32) -> Box<dyn B> {
    if n == 0 {
        Box::new(Atype)
    } else if n == 1 {
        return Box::new(Btype);
    } else {
        return Box::new(Ctype);
    }
}

fn doit4(_a: &dyn B) {}

pub struct S(String);

pub trait Test {
    fn tt(S(t): S) {
        println!("{}", t);
    }
}
