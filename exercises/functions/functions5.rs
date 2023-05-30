// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

// ;가 있으면 그냥 문장이니 무조건 오류를 뱉어냄! ;를 지워줘야  return을 합니다잉~

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
