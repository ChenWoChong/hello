fn main() {
    let p = Point { x: 11, y: 22 };
    display(p.x, p.y);

    let p = Point {
        x: "string",
        y: "yyyy",
    };
    display(p.x, p.y);

    let p = Point { x: 12.3, y: 13.4 };
    display(p.x, p.y);

    // let p = Point{x:A, y:A};
    // display(p.x, p.y);
    let mut f = Football;
    f.play(SportType::Land);
    f.play_mut();
    f.play_own();
    let _f = Football::play_some();
    let _f = <Football as Sport>::play_some();

    doit::<TypeA>("hello".to_string());

    let _f = Foo { x: TypeB };

    println!("{:?}", B::HH);
    println!("{:?}", <B as TraitC>::HH);

    let ta = TA;
    ta.play();
    <TA as Circle>::play(&ta);
    <TA as Shape>::play(&ta);
}

pub trait Iterator {
    type Item;

    fn next(&self) -> Option<Self::Item>;
    fn next2(&self) -> Option<<Self as Iterator>::Item>;
}

// struct A;

trait Sport {
    type SportType;

    fn play(&self, st: Self::SportType);
    fn play_mut(&mut self) {}
    fn play_own(self);
    fn play_some() -> Self;
}

enum SportType {
    Land,
    Water,
}

struct Football;

impl Sport for Football {
    type SportType = SportType;

    fn play(&self, _st: Self::SportType) {}
    fn play_own(self) {}
    fn play_some() -> Self {
        Self
    }
}

trait TraitA {
    type MyType;
}

struct TypeA;
impl TraitA for TypeA {
    type MyType = String;
}

struct TypeB;
impl TraitA for TypeB {
    type MyType = i32;
}

trait TraitB {
    type Item: std::fmt::Debug;
}

#[derive(Debug)]
struct A;

struct B;
impl TraitB for B {
    type Item = A;
}

fn doit2<T>()
where
    T: TraitB,
    T::Item: std::fmt::Debug + std::fmt::Display,
{
}

fn doit<T: TraitA>(_a: T::MyType) {}

struct Foo<T: TraitA<MyType = i32>> {
    x: T,
}

trait TraitC {
    const HH: i32 = 10;
}

impl TraitC for B {
    const HH: i32 = 66;
}

trait TraitD: TraitB + TraitC {}

struct Point<T> {
    x: T,
    y: T,
}

fn display<T: std::fmt::Display>(x: T, y: T) {
    println!("x is {}, y is {}", x, y);
}

trait Shape {
    fn play(&self) {
        println!("1");
    }
}
trait Circle: Shape {
    fn play(&self) {
        println!("2");
    }
}
struct TA;
struct TB;
impl Shape for TA {}
impl Circle for TA {}
impl TA {
    fn play(&self) {
        println!("3");
    }
}

mod module_a {
    pub trait Shape {
        fn play(&self) {
            println!("1");
        }
    }

    pub struct A;
    impl Shape for A {}
}

mod module_b {
    use super::module_a::A;
    // use super::module_a::Shape;
    use crate::module_a::Shape;

    fn doit() {
        let a = A;
        a.play();
    }
}