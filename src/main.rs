fn main() {
    let _t = Option::<u32>::Some(10);

    let x = Some("value");
    assert_eq!(x.expect("test"), "value");
    // let x_none = Option::<String>::None;
    // assert_eq!(x_none.expect("not none"), "none");
    // assert_eq!(x_none.unwrap(), "none");

    // assert_eq!(std::env::var("IMPORTANT_PATH").expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`"), "");
    // assert_eq!(std::env::var("IMPORTANT_PATH").unwrap(), "");
    assert_eq!(Some("car").unwrap(), "car");
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    let default = 2;
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(default), 9);
    assert_eq!(None.unwrap_or(default), default);

    let x: Result<u32, &str> = Err("error");
    assert_eq!(x.unwrap_or(default), default);

    let x: Option<u32> = None;
    let y: Option<u32> = Some(13);

    assert_eq!(x.unwrap_or_default(), 0);
    assert_eq!(y.unwrap_or_default(), 13);

    let gi = "1990";
    let bi = "1990black";
    let gy = gi.parse::<i32>().unwrap_or_default();
    let by = bi.parse::<i32>().unwrap_or_default();

    assert_eq!(gy, 1990);
    assert_eq!(by, 0);

    let mss = Some("Hello, World!".to_string());
    let msl = mss.map(|s| s.len());
    assert_eq!(msl, Some(13));

    let x: Option<&str> = None;
    assert_eq!(x.map(|s| s.len()), None);

    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));

    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(x));

    assert_eq!(cloned.is_some(), true);
    assert_eq!(None::<u32>.is_some(), false);
    assert_eq!(None::<u32>.is_none(), true);

    let text: Option<String> = Some("hello".to_string());
    let tl = text.as_ref().map(|s| s.len());
    println!("len {tl:?} still can use text:{text:?}");

    let mut x = Some(9);
    match x.as_mut() {
        Some(v) => *v = 81,
        None => {}
    }
    println!("{x:?}");
    assert_eq!(x, Some(81));

    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(81));

    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, None);

    let old = x.replace(9);
    assert_eq!(x, Some(9));
    assert_eq!(old, None);

    assert_eq!(Some(2).and_then(sq_to_string), Some(4.to_string()));
    assert_eq!(Some(1_000_000).and_then(sq_to_string), None);
    assert_eq!(None.and_then(sq_to_string), None);

    let line = "1\n2\n3\n4\n";
    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            Ok(n) => println!("{n}"),
            Err(..) => {}
        }
    }

    let x = Ok::<i32, String>(3);
    assert_eq!(x.is_ok(), true);
    assert_eq!(x.is_err(), false);
    assert_eq!(x.as_ref(), Ok(&3));

    let y: Result<i32, i32> = Err(33);
    assert_eq!(y.is_ok(), false);
    assert_eq!(y.is_err(), true);
    assert_eq!(y.as_ref(), Err(&33));

    let mut x = Ok::<i32, i32>(33);
    mutate(&mut x);
    assert_eq!(x.unwrap(), 33 * 33);
    let mut y = Err::<i32, i32>(66);
    mutate(&mut y);
    assert_eq!(y.unwrap_err(), -1);

    assert_eq!(Ok(2).and_then(sqs_to_string), Ok(4.to_string()));
    assert_eq!(Ok(1_000_000).and_then(sqs_to_string), Err("overflowed"));
    assert_eq!(
        Err::<i32, i32>(13).map_err(|v| format!("error code: {v}")),
        Err("error code: 13".to_string())
    );

    let x = Some("foo");
    assert_eq!(x.ok_or("err"), Ok::<&str, &str>("foo"));
    let x = None::<i32>;
    assert_eq!(x.ok_or(0), Err(0));

    let x = Ok::<i32, i32>(3);
    assert_eq!(x.ok(), Some(3));

    let x = Err::<i32, i32>(6);
    assert_eq!(x.ok(), None);

    println!("{}", "-------------");
    let mut a = vec![7, 8, 9];
    let mut a_iter = a.iter_mut();
    while let Some(v) = a_iter.next() {
        println!("{v}");
        *v *= 2;
    }
    println!("{a:?}");

    let mut a_iter = a.iter();
    assert_eq!(Some(&14), a_iter.next());

    let mut a_iter = a.iter_mut();
    assert_eq!(Some(&mut 14), a_iter.next());

    let mut ai_iter = a.clone().into_iter();
    assert_eq!(Some(14), ai_iter.next());
    println!("{:?}", a);

    let a = ["1".to_string()];
    let mut aa = a.clone().into_iter();
    assert_eq!(Some("1".to_string()), aa.next());
    assert_eq!(None, aa.next());
    println!("{:?}", a);

    let mut xxx = [1, 2, 3];
    let mut xxi = xxx.into_iter();
    while let Some(v) = xxi.next() {
        println!("{v}");
    }
    println!("{xxx:?}");

    for num in xxx {
        println!("{num}");
    }
    for num in &xxx {
        println!("{}", *num);
    }
    for num in &mut xxx {
        *num *= 2;
        println!("{}", num);
    }

    let s1 = String::from("111");
    let s2 = String::from("222");
    let s3 = String::from("333");
    let sv = vec![s1, s2, s3];
    let _a= &sv[0];
    let _b= sv[0].clone();

    for s in sv {
        println!("{s}");
    }
}

fn mutate(x: &mut Result<i32, i32>) {
    match x.as_mut() {
        Ok(v) => *v *= *v,
        Err(e) => *e = -1,
    }
}

fn sq_to_string(x: u32) -> Option<String> {
    x.checked_mul(x).map(|s| s.to_string())
}

#[allow(dead_code)]
enum MyOption<T> {
    None,
    Some(T),
}

pub enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn sqs_to_string(x: i32) -> Result<String, &'static str> {
    x.checked_mul(x).map(|v| v.to_string()).ok_or("overflowed")
}
