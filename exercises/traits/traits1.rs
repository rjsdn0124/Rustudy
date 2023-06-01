// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar` for the type `String`.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a hint.

// trait을 정의하면 약간 인터페이스 느낌으로 어떤 struct 같은 거에 해당 함수를 추가할 수 있다.
// trait 키워드를 쓰는구만!

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) -> Self{
        // String은 참조값이 아니라 그대로 들어간다잉.
        // 이렇게 하면 새로 객체가 생기는게 아니라, self.Add 함수를 호출한거니 add함수도 self를 반환하니까 괜찮네!
        self + "Bar"
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
