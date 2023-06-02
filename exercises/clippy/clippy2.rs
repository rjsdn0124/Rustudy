// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

// 아 더 깔끔한 코드도 추천을 해주네!
// if let으로 바꾸는게 더 좋다고!

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
