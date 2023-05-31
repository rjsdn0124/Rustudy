// enums1.rs
// No hints this time! ;)

// 객체처럼 사용할 수 있는 거 같은데? 약간 json 객체 느낌. 아하
// 아하 Quit =1, echo=2이런 식으로 초기화 하는게 아니라 여기서 구현한거처럼 이렇게 사용해서
// 그냥 하.. 모르겠다.. 상속처럼 쓰는거 같은데 enum안의 필드들은 객체처럼 쓰이는거 같은데..? 구조체의 모임 느낌..?
// 그렇네 3가지 유형의 구조체를 모두 넣을 수 있네. 구조체의 interface느낌이네


#[derive(Debug)]
enum Message {
    Quit, Echo, Move, ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
