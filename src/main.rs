fn main() {
    let _a = foo();
    let _a = foo1();
    let _a = foo2();

    let ba = foo1();
    let a = *ba;
    println!("{:?}", ba);
    println!("{:?}", a);

    let mut ba = foo2();
    ba.play();
    *ba = Point{ x: 100, y: 100 };
    ba.play();
    let a = *ba;
    // println!("{:?}", ba);
    println!("{:?}", a);
}

fn foo() -> String {
    let a = "abc".to_string();
    a
}

fn foo1() -> Box<u32> {
    let n = 10u32;
    let boxed = Box::new(n);
    let _m = n;
    let _bb = boxed.clone();
    boxed
}
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn play(&self) {
        println!("Point x: {}, y: {}", self.x, self.y);
    }
}

fn foo2() -> Box<Point> {
    let p = Point { x: 1, y: 1 };
    let boxed = Box::new(p);
    // let q = p;
    boxed
}
