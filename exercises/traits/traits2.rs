// traits2.rs
//
// Your task is to implement the trait
// `AppendBar` for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(&mut self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    // 벡터를 다시 쓰니가 vec 다시 익숙해졌다!
    // vec를 주소값으로 사용하고나서는 tovec으로 반환해주자!
    // 그냥 self를 사용하면 참조값이 아닌 vec 그대로 드어오는데 self를 다시 반환해주니까 주소값이 무너지지 않고 그대로 작성이되어서 들어간다!
    // mut를 사용하려면 참조값이랑 함께 두과자~
    fn append_bar(&mut self) -> Self{
        self.push(String::from("Bar"));
        self.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
