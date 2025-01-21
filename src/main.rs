fn main() {
    doit1(A);
    doit2(A);
    doit3(A);
    doit4(A);
    doit5(A);
    doit6(A);
    doit7(A);

    doit4(B);
    doit6(B);

    doit7(C);

    let p = Pair::<i32> { x: 3, y: 6 };
    p.cmp_display();
}

struct Pair<T> {
    x: T,
    y: T,
}

// impl std::fmt::Display for i32 {
//     fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Ok(())
//     }
// }

impl<T> Pair<T>
where
    T: std::fmt::Display + std::cmp::PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest num is {}", self.x);
        } else {
            println!("the largest num is {}", self.y);
        }
    }
}

impl<T: TraitB> TraitA for T {}

impl TraitA for u32 {}

trait TraitA {}
trait TraitB {}
trait TraitC {}

struct A;
struct B;
struct C;

impl TraitA for A {}
impl TraitB for A {}
impl TraitC for A {}

impl TraitB for B {}
impl TraitC for B {}

impl TraitC for C {}

fn doit1<T: TraitA + TraitB + TraitC>(_t: T) {}
fn doit2<T: TraitA + TraitB>(_t: T) {}
fn doit3<T: TraitA + TraitC>(_t: T) {}
fn doit4<T: TraitB + TraitC>(_t: T) {}
fn doit5<T: TraitA>(_t: T) {}
fn doit6<T: TraitB>(_t: T) {}
fn doit7<T: TraitC>(_t: T) {}
