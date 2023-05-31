// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.

// 그냥 문자열을 하면 'static 선언을 해줘야하는데 static이니까 별로 안좋아. 그래서 to_string으로 변환해서 관리 가능하도록 하는게 good way인듯!

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}
