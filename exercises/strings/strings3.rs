// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

// 아 웬만하면 변수.함수 사용할 때는 변수가 참조값이어야하넹. 상관은 엇는거 같기도 하고..?
// Some()은 그냥 Option enum의 결과로 나오면 Some이 붙는거. 적용된 함수가 option enum형태를 반환한다는 뜻!
// trim은 &str 반환하고 replace는 String 반환하고 String형태에 &str을 더하면 String이 되넹. 규칙을 모르겠다.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    
    // String.Add 방법으로 접근하는 벙법!
    // input.to_string() + " world!"
    
    // std::format 방법으로 접근하는 방법! 밑에게 조금 더 가시성도 좋고 괜찮아보이는데? 
    // format이랑 Add의 작동 방식에 차이를 정확히 몰라서 성능은 모르겠는데 밑에게 더 좋아보이긴 하네
    format!("{} {}",input, "world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("car","balloon")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
