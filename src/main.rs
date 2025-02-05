fn main() {
    let _c = Color::default();
    let _c = <Color>::default();
    let _c: Color = Default::default();
    let _c = Some(Color::default());

    let p = Point::default();
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
    println!("{}", p);
    let ps = format!("{}", p);
    assert_eq!(ps, "0 0");
    println!("{:#?}", p);

    let p = <Point>::default();
    let q = <Point>::default();
    assert_eq!(teq(p, q), true);
    assert_eq!(teqc(vec![p], vec![q]), true);

    example_bteeset();
    example_ord(vec![<Point>::default(), Point { x: 1, y: 1 }]);

    let p = Point { x: 3, y: 8 };
    let q = Point { x: 1, y: 1 };
    println!("{}", p + q);
    println!("{}, {}", p, p + 3);
    println!("{}", p);

    let a = Atype {
        num: 10,
        avec: vec![10, 10, 10],
    };

    let mut b = a.clone();
    b.num = 1000;
    b.avec[0] = 11;
    b.avec[1] = 22;
    b.avec[2] = 33;
    println!("{:?}", a);
    println!("{:?}", b);

    let a = "123456";
    let b = a.to_owned();
    println!("{:?}", b);

    let range = 0..10;
    let cnt = range.len();
    println!("{}", cnt);
    let get_cnt = || range.count();
    assert_eq!(get_cnt(), 10);

    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let mut min = i32::MIN;
    let ascending = nums
        .into_iter()
        .filter(|&n| {
            if n <= min {
                false
            } else {
                min = n;
                true
            }
        })
        .collect::<Vec<i32>>();
    println!("{:?}", ascending);

    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let min = 9;
    let gt_9 = nums.into_iter().filter(|&n| n <= min).collect::<Vec<_>>();
    println!("{:?}", gt_9);
    println!("{:?}", gt_9);

    let mut fp: fn(i32) -> i32 = add_one;
    assert_eq!(fp(1), 2);

    fp = |x| x * x;
    assert_eq!(fp(3), 9);

    let p = Point::from((8, 8));
    println!("{}", p);
    let p = Into::<Point>::into((9, 9));
    println!("{}", p);

    let p = vec![Color(0, 0, 0)];
    println!("{}", p.len());
    let p = guarantee_length(p, 10);
    println!("{}", p.len());
}

pub trait Default {
    fn default() -> Self;
}

fn guarantee_length<T: Default>(mut vec: Vec<T>, min_len: usize) -> Vec<T> {
    for _ in 0..min_len.saturating_add(vec.len()) {
        vec.push(T::default());
    }
    vec
}

#[allow(dead_code)]
pub struct Color(u8, u8, u8);
impl Default for Color {
    fn default() -> Self {
        Color(0, 0, 0)
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    x: u8,
    y: u8,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

pub trait D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point() {
        let p = Point::default();
        assert_eq!(format!("{}", p), "0 0");
        assert_eq!(format!("{}", p), ToString::to_string(&p));
        assert_eq!(format!("{}", p), std::string::ToString::to_string(&p));
    }

    #[test]
    fn test_pp() {
        assert_eq!(format!("{}", <Point>::default()), "0 0");
    }
}

fn teq(p: Point, q: Point) -> bool {
    p == q
}

fn teqc<T: PartialEq>(p: Vec<T>, q: Vec<T>) -> bool {
    p == q
}

fn example_bteeset() {
    let mut bs = std::collections::BTreeSet::<Point>::new();
    let p = <Point>::default();
    bs.insert(p);
}

fn example_ord<T: Ord>(mut sorts: Vec<T>) -> Vec<T> {
    sorts.sort();
    sorts
}

pub trait Ad<Rhs = Self> {
    type Output;
    fn add(&self, rhs: Rhs) -> Self::Output;
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<i32> for Point {
    type Output = Point;
    fn add(mut self, rhs: i32) -> Self::Output {
        self.x += rhs as u8;
        self.y += rhs as u8;
        self
    }
}

pub trait Cl {
    fn clone(&self) -> Self;
}

pub trait Cp: Cl {}

#[derive(Clone, Debug)]
struct Atype {
    num: i32,
    avec: Vec<u32>,
}

pub trait MyDeref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

pub trait MyDrop {
    fn drop(&mut self);
}

pub trait MyFnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

pub trait MyFnMut<Args>: MyFnOnce<Args> {
    // type Output;
    fn all_mut(&mut self, args: Args) -> Self::Output;
}

pub trait MyFn<Args>: MyFnMut<Args> {
    // type Output;
    fn call(&self, args: Args) -> Self::Output;
}

pub fn add_one(n: i32) -> i32 {
    n + 1
}

pub trait MyFrom<T> {
    fn from(t: T) -> Self;
}

pub trait MyInto<T> {
    fn into(self) -> T;
}

impl<T, U> MyInto<U> for T
where
    U: MyFrom<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}

impl From<(u8, u8)> for Point {
    fn from((x, y): (u8, u8)) -> Self {
        Point { x, y }
    }
}

pub trait MyTryFrom<T> {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait MyTryInto<T> {
    type Error;
    fn tyr_into(self) -> Result<T, Self::Error>;
}

pub trait MyFromStr {
    type Error;
    fn from_str(str: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub fn example_from_str<T: std::str::FromStr>(t: &str) {
    let _tmp: Result<T, _> = std::str::FromStr::from_str(t);
    let _tmp = T::from_str(t);
    let _tmp: Result<T, _> = t.parse();
    let _tmp = t.parse::<T>();
}

pub trait MyAsRef<T> {
    fn as_ref(&self) -> &T;
}

fn take_str(_s: &str) {}

fn take_asref_str(_s: impl AsRef<str>) {}

pub fn test_as_ref(str: &str, borrow: &String, owned: String) {
    take_str(str);
    take_str(borrow);
    // take_str(owned);
    take_asref_str(str);
    take_asref_str(borrow);
    take_asref_str(owned);
}
