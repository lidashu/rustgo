macro_rules! printmany {
    ( $( $x:expr ),* ) => {
        {
            $(
                println!("print this");
                println!($x);
            )*
        }
    };
}

fn main() {
    printmany!("Hello, world!", "Hello, world!", "Hello, world!");
}
