// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

// 참조자만 사용해서 코드 수정하기. getchar에서는 data를 이후에도 써야하니 주소값 참조를 했고, 이 후에는 필요 업서지니 그냥 참조를 지웠다.
// 이게 맞는지는 모르겠지만 data에 string을 넣으려면 이 방법 뿐인 것 같다. 주소값 참조로 인자를 받으면 주소값으로 다시 넣어줘야하는데 그게 안되네..?
// 아 string_uppercase 주석에 ownership을 가져도 된다고 나와있넹ㅎ 이게 맞네

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);
    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
