use std::sync::Arc;

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
    *ba = Point { x: 100, y: 100 };
    ba.play();

    let mut anb = ba.clone();
    *anb = Point { x: 4, y: 6 };
    anb.play();
    let y = &anb;
    y.play();
    let my = &mut anb;
    **my = Point { x: 3, y: 5 };
    anb.play();

    let a = *ba;
    // println!("{:?}", ba);
    println!("{:?}", a);

    let mut ba = foo2();
    ba.play_ref();
    ba.play_mutref();
    ba.play_own();
    let ba = foo2();
    ba.play_boxed();

    let a = Box::new(Atype {});
    doit(a);
    let b = Box::new(Btype {});
    doit(b);
    let c = Box::new(Ctype {});
    doit(c);

    let _i = Mystruct {
        a: Box::new(Atype {}),
    };
    let _i = Mystruct {
        a: Box::new(Btype {}),
    };
    let _i = Mystruct {
        a: Box::new(Ctype {}),
    };
    let _b = Mystruct {
        a: Box::new(Ctype {}),
    };

    let a = std::sync::Arc::new(Point { x: 1, y: 2 });
    let an = a.clone();
    a.play();
    an.play();
    let ann = an.clone();
    ann.play();

    let c = std::sync::Arc::new(Point { x: 1, y: 2 });
    c.play();
    c.play_ref();
    // c.play_mutref();
    // c.play_own();
    c.play_arc();
    // c.play();
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
#[derive(Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn play(&self) {
        println!("Point x: {}, y: {}", self.x, self.y);
    }

    fn play_ref(&self) {
        println!("Point x: {}, y: {}", self.x, self.y);
    }

    fn play_mutref(&mut self) {
        println!("Point x: {}, y: {}", self.x, self.y);
    }
    fn play_own(self) {
        println!("Point x: {}, y: {}", self.x, self.y);
    }
    fn play_boxed(self: Box<Self>) {
        println!("Point x: {}, y: {}", self.x, self.y);
    }

    fn play_arc(self: Arc<Self>) {
        println!("Point x: {}, y: {}", self.x, self.y);
    }
}

fn foo2() -> Box<Point> {
    let p = Point { x: 1, y: 1 };
    let boxed = Box::new(p);
    // let q = p;
    boxed
}

struct Atype {}
struct Btype {}
struct Ctype {}

struct Mystruct {
    #[allow(dead_code)]
    a: Box<dyn Atrait>,
}

pub struct Mystruct2<'a> {
    #[allow(dead_code)]
    a: &'a dyn Atrait,
}

#[derive(Debug)]
pub struct Point2 {
    x: i32,
    y: i32,
}

trait Atrait {}
impl Atrait for Atype {}
impl Atrait for Btype {}
impl Atrait for Ctype {}

fn doit(_t: Box<dyn Atrait>) {}
