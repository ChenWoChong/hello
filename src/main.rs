#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
fn main() {
    let mut a=10u32;
    let mut b = &mut a;
    let mut c = &mut b;
    let d = &mut c;
    ***d = 88;
    println!("{d}");

    let mut s = String::from("hh");
    println!("{s}");
    foo(&mut s);
    println!("{s}");

    let mut a = 10u32;
    let b = &a;
    println!("{b}");
    a=33;
}

fn foo(s: &mut String) {
    s.push_str("hello world.");
}