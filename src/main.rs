fn main() {
    let _ = Shape::Triangle(Triangle {
        width: 1,
        height: 1,
    });
    let _ = Shape::Rectangle((1, 1), (1, 1), (1, 1));
    let _ = Shape::Circle {
        origin: (1, 1),
        radius: 1,
    };

    let _a = WebEvent::PageLoad;
    let _a = WebEvent::PageUpdate;
    let _c = WebEvent::KeyPress('c');
    let _d = WebEvent::Paste(String::from("hh"));
    let _e = WebEvent::Click { x: 1, y: 1 };

    println!("zero is {}", Number::Zero as i32);
    println!("ten is {}", Number::Ten as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("leaves are #{:04x}", Color::Green as i32);
    println!("violets are #{:04x}", Color::Blue as i32);
    // let _ = Foo {}; // error

    let add = MyEnum::Add;
    println!("{}", add.run(111, 222));
    println!("{}", MyEnum::Subtract.run(333, 55));

    let shape_a = Shape::Rectangle((22, 22), (33, 33), (44, 44));
    let shape_a = Shape::Circle {
        origin: (44, 44),
        radius: 8,
    };
    let shape_a = Shape::Triangle(Triangle {
        width: 88,
        height: 99,
    });
    let ret_a = match shape_a {
        Shape::Rectangle(x, y, z) => {
            println!("{:?}, {:?}, {:?}", x, y, z);
            1
        }
        Shape::Triangle(x) => {
            println!("{:?}", x);
            2
        }
        Shape::Circle {
            origin: x,
            radius: y,
        } => {
            println!("{:?}, {:?}", x, y);
            3
        }
        _ => 88,
    };
    println!("{ret_a}");

    let number = 22;
    println!("Tell me about {number}");

    match number {
        1 => println!("One!"),
        2 | 3 | 4 | 5 | 6 => println!("greater than one!"),
        5..19 => println!("Big"),
        _ => println!("Other"),
    }

    let my_enum = MyEnum::Subtract;
    if let MyEnum::Add = my_enum {
        println!("1");
    } else {
        println!("2");
    }

    let mut num = 9;
    let mut enum_a = MyEnum::Div;
    while let MyEnum::Div = enum_a {
        if num >= 0 {
            println!("Greater than 0 {}", num);
            num -= 1;
        } else {
            println!("Less    than 0 {}", num);
            enum_a = MyEnum::Add;
        }
    }

    let shape_a = Shape::Triangle(Triangle {
        width: 12,
        height: 13,
    });
    let Shape::Triangle(Triangle { width, height }) = shape_a else {
        panic!("can't extract triangle..");
    };
    println!("width: {}, height: {}", width, height);

    let a = (1, 'a', "hahah");
    let (b, c, d) = a;
    println!("{b} {c} {d}");

    let mut v = vec![Some(1), Some(2), Some(3)];

    while let Some(Some(i)) = v.pop() {
        println!("got {}", i);
    }

    let refs = &42;
    match refs {
        &v => println!("refs points to: {}", v),
    }

    let mut stu = Student {
        name: "Chen".to_string(),
        age: 30,
        student: false,
    };

    let Student {
        ref mut name,
        age,
        student: students,
    } = stu;
    println!("name {}", name);
    *name = "wochong".to_string();
    println!("name {}", name);
    println!("age {}", age);
    println!("student {}", students);
    println!("{:?}", stu);

    let tmp = (1,2,'c');
    foo(tmp);

    fooStu(stu);
}

#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
    student: bool,
}

#[derive(PartialEq, Debug)]
enum MyEnum {
    Add,
    Subtract,
    Div,
}

impl MyEnum {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Div => x + y,
        }
    }
}

enum Foo {}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum Number {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Night,
    Ten,
    Eleven,
    Twelve,
}

enum Shape {
    Triangle(Triangle),
    Rectangle((u32, u32), (u32, u32), (u32, u32)),
    Circle { origin: (u32, u32), radius: u32 },
    Pie,
}

#[derive(Debug)]
struct Triangle {
    width: u32,
    height: u32,
}

enum WebEvent {
    PageLoad,
    PageUpdate,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn foo((a, b, c): (u32, u32, char)) {
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

fn fooStu(Student{name, age,student}: Student) {
    println!("{name}");
    println!("{age}");
    println!("{student}");
}
