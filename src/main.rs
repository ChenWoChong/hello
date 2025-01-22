fn main() {
    assert_eq!(doit(0), Ttype::A(Atype));
    assert_eq!(doit(1), Ttype::B(Btype));
    assert_eq!(doit(2), Ttype::C(Ctype));
}

#[derive(Debug, PartialEq)]
struct Atype;
#[derive(Debug, PartialEq)]
struct Btype;
#[derive(Debug, PartialEq)]
struct Ctype;

#[derive(Debug, PartialEq)]
enum Ttype {
    A(Atype),
    B(Btype),
    C(Ctype),
}

fn doit(num: i32) -> Ttype {
    if num == 0 {
        let a = Atype;
        return Ttype::A(a);
    } else if num == 1 {
        let b = Btype;
        return Ttype::B(b);
    } else {
        let c = Ctype;
        return Ttype::C(c);
    }
}
