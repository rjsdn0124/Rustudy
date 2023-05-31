// enums3.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a hint.

// 와 쥰내 어려웠다. match를 사용하는 거였는데 match 성능ㅇ ㅣ무쳤다.해당하는 enum의 형태를 인자까지 만족하면, 해당하는 라인을 실행하는데 그 인자들을 다시 사용하는 방식이다.
// rust.. 미쳤네..?

enum Message {
    // TODO: implement the message variant types based on their usage below
    Color (u8,u8,u8),
    Contents (String),
    Point (Point),
    Quit,
}

impl Message {
    fn Move(point:Point) -> Message {
        Message::Point(point)
    }
    fn Echo(temp:String) -> Message{
        Message::Contents(temp)
    }
    fn ChangeColor(a:u8,b:u8,c:u8) -> Message{
        Message::Color(a,b,c)
    }
}
struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        // Remember: When passing a tuple as a function argument, you'll need extra parentheses: fn function((t, u, p, l, e))
        match message{
            Message::Color(r,g,b) => self.change_color((r,g,b)),
            Message::Contents(text) => self.echo(text),
            Message::Point (point) => self.move_position(point),
            Message::Quit => self.quit()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
