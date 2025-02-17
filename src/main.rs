#[macro_export]
macro_rules! add {
    ($a:expr, $b:expr) => {{
        $a + $b
    }};
    ($a:expr) => {{
        $a + $a
    }};
    ($($a:expr),*) => {{

        0
        $( + $a )*
    }
    };
}

fn main() {
    println!("{}", add!(2));
    println!("{}", add!(1, 2));
    println!("{}", add!(1, 2, 3));
    println!("{}", add!(1, 2, 3, 4));

    m![];
    crate::m!();
    self::add!();
    crate::add!();
}

mod toexport {
    #[macro_export]
    macro_rules! m {
        () => {};
    }
}

mod inner {
    super::m!();
    crate::m!();
}
