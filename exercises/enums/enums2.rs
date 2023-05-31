// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

// 흠... message를 이렇게 나눠서 하니까 감이 잘 안오네... 다음 거에서 해보자!
// 추가로 안 사실은 for문에서도 참조를 하면 변수 메모리가 날라감!

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move {x:i32, y:i32},
    Contents (String),
    Color (i32,i32,i32),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
    fn Echo(temp:String) -> Message{
        Message::Contents(temp)
    }
    fn ChangeColor(a:i32,b:i32,c:i32) -> Message{
        Message::Color(a,b,c)
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in messages {
        message.call();
    }
}
