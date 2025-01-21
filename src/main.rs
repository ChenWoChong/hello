fn main() {
    let _ta = Atype::<i32> { a: 8 };

    let p = Point { x: 1, y: 2 };
    p.echo();
    let np = p.add(Point { x: 3, y: 4 });
    np.echo();
    let np = p.add(38);
    np.echo();
}

trait TraitA<T> {}

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
