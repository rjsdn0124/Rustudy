// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

// Rust는 변수 안의 값을 수정할 수 없어요.. 대신 let mut 키워드를 이용해서 변수를 바꿀 수 있는 변수로 만들어줄 수 있어욥!

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
