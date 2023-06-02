// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

// 매크로 만들기! macro_rules! 를 붙이고 함수를 만들고 해당 함수 내엔 콜백함수 형태로 내용을 채워준다!
// 그리고 사용은 늘 그렇듯이 ! 붙여서

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
