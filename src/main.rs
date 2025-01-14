fn main() {
    let mut a = String::from("hello");
    let mut b = "halo".to_string();

    let mut c = || {
        a.push_str(" world!");
        println!("c {}", a);
    };

    let mut c1 = move || {
        b.push_str("!!@");
        println!("c1 {b}");
    };

    c();
    c1();

    c();
    c1();
    println!("--------------");

    call_mut(&mut c);
    call_mut(&mut c1);
    call_mut(&mut c);
    call_mut(&mut c1);
    println!("--------------");


    call_once(c);
    call_once(c1);
    // 不可以调用两次
    // call_once(c); 
    // call_once(c1);

}

fn call_mut(c: &mut impl FnMut()) {
    c();
}

fn call_once(c: impl FnOnce()) {
    c();
}
