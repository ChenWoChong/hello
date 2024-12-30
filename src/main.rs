use std::rc::Rc;

fn main() {
    types_impl_copy_trait()
}

fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    // is_copy::<String>();

    is_copy::<fn(Rc<i32>)>();

    is_copy::<*const String>();
    is_copy::<*mut String>();

    is_copy::<&[Vec<u8>]>();
    // is_copy::<[Vec<u8>]>();

    is_copy::<[u8; 4]>();
    // is_copy::<[String; 4]>();

    is_copy::<(&str, &str)>();
    // is_copy::<(str, str)>();
}
