// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

// 매크로 이름으로 오버라이드할 수 있는데 서로 구분해주기 위해선 ;이 필요하단다. 굳이 이걸 공부할 필요가..?

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ( $val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
