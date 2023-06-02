// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

// 한 줄로 구현하는게 개 섹시하긴 하네...ㅎ
// 근데 fold 함수가 있다는거 까지는 ㅇㅈ..
// 왜 그냥 iter 하면 rangeInclusive 로 반환이 되는거지??
// 아 [1..=num] 하면 배열 안에 rangeinclusive 객체가 들어가서 원소가 하나인 rangeInclusive객체가 되는거네
// 원래 사용은 (1..=num)하면 rangeinclusive 객체를 만들어줘용 이네. 1..=num 이 구문이 해당 객체 만들어주는거고 이거를 괄호로 감싼거네.
// rangeinclusive는 iter가 없어서 into_iter을 서야함!

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    if num == 0 {1} else {
        (1..=num).into_iter().fold(1,|save, next| save * next )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
