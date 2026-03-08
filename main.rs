macro_rules! my_macro {
    ($x:expr) => {
        println!("The value is: {}", $x);
    };
}

fn main() {
    my_macro!(42);
}
