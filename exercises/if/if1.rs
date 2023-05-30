// if1.rs
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

// pub로 public 함수를 만들어주넹. 여기선 pub없어도 실행이 되긴 한다!
// match라는 조건을 판별하는 키워드도 있다! case문이랑 비슷한건듯! match a {1=> 1, _=> 2,} - a가 1이면 1을 반환. 그 외라면 2를 반환.

pub fn bigger(a: i32, b: i32) -> i32 {
    // 3항 연산자처럼 쓰이는 if문. 한 줄로 if문을 정의할 수있음. 파이썬이랑 비슷한듯?
    if a > b {a} else {b}
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
