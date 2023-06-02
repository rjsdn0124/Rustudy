// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// 매크로 선언하고 사용할거면 사용될 위치의 항상 위에!

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
