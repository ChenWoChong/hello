use std::{slice, str};
#[allow(static_mut_refs)]
fn main() {
    let mut u = IntOrFloat { f: 1.0 };
    assert_eq!(unsafe { u.i }, 1065353216);
    u.i = 1073741824;
    assert_eq!(unsafe { u.f }, 2.0);

    let v = [1, 2, 3];
    add_counter(6);
    unsafe {
        println!("COUNTER: {}", COUNTER);
        // println!("{}", v[3]);
        println!("v[2]: {}", v[2]);
    }

    let num: i32 = 10;
    let num_ptr: *const i32 = &num;
    let mut nu: i32 = 11;
    let mut_nu_ptr: *mut i32 = &mut nu;
    unsafe {
        println!("num {}", *num_ptr);
        println!("nu {}", *mut_nu_ptr);
    }

    let tt = Box::new(66);
    let tt = Box::into_raw(tt);
    unsafe {
        let _ = Box::from_raw(tt);
    }

    use std::ptr;

    let null_v: *const i32 = ptr::null();
    assert!(null_v.is_null());

    let mut_null: *mut i32 = ptr::null_mut();
    assert!(mut_null.is_null());

    aa();

    let mut ass = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("ass:\t {:?}", ass);
    let (l, r) = split_slice(&mut ass, 3);
    println!("left:\t {:?}", l);
    println!("right:\t {:?}", r);
    println!("ass:\t {:?}", ass);

    unsafe {
        assert_eq!(ass.get_unchecked(2), &3);
    }

    let heart = vec![240, 159, 146, 150];

    let heart_str = unsafe { str::from_utf8_unchecked(&heart) };
    assert_eq!("💖", heart_str);
    println!("heart:\t {}", heart_str);
}

fn aa() {
    let a = &10 as *const i32;
    let b = &mut 11 as *mut i32;

    unsafe {
        println!("a {}", *a);
        println!("b {}", *b);
    }
}
union IntOrFloat {
    i: u32,
    f: f32,
}

static mut COUNTER: i32 = 30;

fn add_counter(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

fn split_slice(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = v.len();
    let ptr = v.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
