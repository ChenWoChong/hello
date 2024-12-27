fn main() {
    let a_vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
    let a_slice = &a_vec[0..5];
    let _aa_vec = a_slice.to_vec();
    let _aaa_vec = a_slice.to_owned();

    let s = String::from("foo");
    assert_eq!(s.as_str(), &s[..]);

    assert_eq!(s.as_bytes(), &[102, 111, 111]);
    assert_eq!(&[1, 2, 3, 4, 5, 6], a_vec.as_slice());
    let v: Vec<u32> = vec![1, 3, 3, 4];
    foo(&v);
    let a = v.as_slice();
    foo(a);
}

fn foo(s: &[u32]) {
    println!("{:?}", s);
}
