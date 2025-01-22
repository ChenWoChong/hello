fn main() {
    let _ta = Atype::<i32> { a: 8 };

    let p = Point { x: 1, y: 2 };
    p.echo();
    let np = p.add(Point { x: 3, y: 4 });
    np.echo();
    let np = p.add(38);
    np.echo();

    let c = Ctype;
    c.play(10u32);
}

trait TraitA<T = Self> {
    fn func(_t: T) {}
}

trait TraitB<T = i32> {
    fn func2(_t: T) {}
}

struct SomeType;
impl TraitA for SomeType {
    fn func(_t: SomeType) {}
}

impl TraitB for SomeType {
    fn func2(_t: i32) {}
}

impl TraitA<String> for SomeType {
    fn func(_t: String) {}
}

impl TraitB<String> for SomeType {
    fn func2(_t: String) {}
}

struct AType {}
impl TraitA<i32> for AType {}

struct Atype<U> {
    a: U,
}
impl<T, U> TraitA<T> for Atype<U>
where
    T: std::fmt::Debug,
    U: PartialEq,
{
}

trait Add<T> {
    type Output;
    fn add(&self, hrs: T) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(&self, hrs: Point) -> Self::Output {
        Point {
            x: self.x + hrs.x,
            y: self.y + hrs.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Self;
    fn add(&self, hrs: i32) -> Self::Output {
        Point {
            x: self.x + hrs,
            y: self.y + hrs,
        }
    }
}

impl Point {
    fn echo(&self) {
        println!("x is {}, y is {}", self.x, self.y);
    }
}

use std::fmt::{Debug, Display};

trait TraitC<T>
where
    T: Debug,
{
    fn play(&self, _t: T);
}

struct Ctype;

impl<T> TraitC<T> for Ctype
where
    T: Debug + Display,
{
    fn play(&self, _t: T) {}
}
