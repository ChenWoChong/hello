use std::collections::HashMap;

fn main() {
    let a = 1.0f32;
    let b = 10 as f32;
    let _ = a * b;

    let t = 9 + 'a' as u8;
    println!("{t} {}", 9.to_string() + "11");

    let ip = Point::<u32, u32> { x: 1, y: 10 };
    let fp = Point::<f32, f32> { x: 1.5, y: 3.0 };
    let cp = Point::<f64, u32> { x: 1, y: 1.5 };

    print(&ip);
    print(&fp);
    print(&cp);
    println!("point sum: {}", ip.sum());
    println!("point x {}", ip.x());
    let mp = cp.mix(Point::<u32, f64> { x: 1.1, y: 3 });
    print(&mp);

    let _eee = EEE::default();

    let _hhh = <HHH>::default();
    let _mytype = <MyType::<i32>>::default();
}

type AAA = HashMap<String, Vec<u8>>;
type BBB = Vec<AAA>;
type CCC = HashMap<String, BBB>;
type DDD = Vec<CCC>;
type EEE = HashMap<String, DDD>;

struct FFF {
    hashmap: HashMap<String, EEE>,
}

struct GGG(FFF);

type HHH = HashMap<String, GGG>;

type MyType<T> = HashMap<String, Vec<HashMap<String, Vec<HashMap<String, Vec<T>>>>>>;

struct Point<T, A> {
    x: A,
    y: T,
}

impl<T, A> Point<T, A> {
    fn mix<X1, X2>(self, other: Point<X1, X2>) -> Point<T, X2> {
        Point {
            x: other.x,
            y: self.y,
        }
    }
}

impl<T: std::ops::Add<Output = T> + Copy> Point<T, T> {
    fn sum(&self) -> T {
        self.x + self.y
    }
}

impl<T> Point<T, T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}

impl Point<u32, u32> {
    fn doit() {}
}

enum MyOption<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    OK(T),
    Err(E),
}

enum Aaa<T, A, U> {
    V1(Point<T, A>),
    V2(Vec<U>),
}

fn print<T: std::fmt::Display, A: std::fmt::Display>(p: &Point<T, A>) {
    println!("Point {}, {}", p.x, p.y);
}

struct List<T>(Vec<T>);

struct Po(u32, u32);

struct Rectangle {
    p1: Po,
    p2: Po,
}

struct Triangle(Po, Po, Po);

struct Circle(Po, u32);

enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}

struct Axes;

struct Geometry {
    shape: Shape,
    axes: Axes,
}

struct Algebra;

enum level {
    Elementary,
    Secondary,
    High,
}

enum Course {
    Geometry(Geometry),
    Algebra(Algebra),
}

struct MathLesson {
    math: Course,
    level: level,
}
