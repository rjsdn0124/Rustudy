// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

// 객체 구조 분해도 되네용. ()안에 받을 변수를 넣으면 이렇게 됩니다. 대신 원하는거만 받는게 아니라 다 받아야ㅑ해요.
// tuple이라고 생각하면 되겠네요.
// 얘네를 참조하는거는 cat.0 cat.1이렇게도 사용할 수 있답니다~
// 이건 번외인데, 배열을 선언할 때는 let array:[i32;5]하면 int형 길이가 5인 배열을 선언할 수 있대용~ 또 let arr = [3;5]하면 3으로 5개 만들어준대용

fn main() {
    let cat:(&str,f64) = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
    println!("{} is {} years old.", cat.0, cat.1);
}
