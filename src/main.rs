fn main() {
    let u = User {
        active: false,
        username: String::from("haha"),
        email: String::from("sdsf"),
        sign_in_count: 88,
    };
    println!("{:#?}", u);

    let u2 = User {
        username: String::from("ttt"),
        email: String::from("xxxx"),
        ..u
    };

    let members = vec![u, u2];
    let c = Class{
        s_num:6,
        g_num:6,
        year: String::from("2025"),
        members,
    };
    println!("{:#?}", c);

    let _color = Color(0,0,0);
    let _mi32 = Myi32(1);
    
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Class {
    s_num: u32,
    g_num: u32,
    year: String,
    members: Vec<User>,
}

struct Color(i32,i32,i32);
struct Myi32(i32);
