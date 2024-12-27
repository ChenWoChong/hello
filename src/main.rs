#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
fn main() {
    let s1: &'static str = "hello";
    let s2 = s1.to_string();
    let s5 = &s2[..6];
    // println!("{s5}");
}