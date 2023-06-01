// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

// impl에서 wrapper에 들어갈 T를 가져오고 그 전에 value에서 T를 받아와서 모두 다 초기화 해준다!
// struct Wrapper<T>는 진짜 struct에 해당하는 거 선언하고 끝! 여기서 T는 다른데의 T랑 다르다
// imple에서 wrapper를 value로부터 받아온 T1타입을 받아오고 전부 초기화 해준다.
// 지금의 함수에서는 모두 T1으로 같이해서 초기화시켜줘야한다!

struct Wrapper<T> {
    value: T,
}

// 이거랑 밑에거랑 같음!
// impl<T> Wrapper<T> {
//     pub fn new(value: T) -> Self {
//         Wrapper { value }
//     }
// }
impl<T1> Wrapper<T1> {
    pub fn new(value: T1) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
