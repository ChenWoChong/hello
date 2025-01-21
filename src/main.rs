fn main() {
    let t = Some("foo");
    assert_eq!(t.ok_or(Err::<&str, &str>("kkk")), Ok("foo"));
}