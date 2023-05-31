// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

// While let, if let을 배웠다.
// if문 조건 안에 변수를 할당을 하는거도 가능한 거 같다! 근데 일반 할당은 어차피 항상 true이니 쓸 일이 없을 걱같고,
// enums 변수 할당할 때는 좋을 거 같다!

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }
        optional_integers.push(None);

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`s into while let and if let
        while let Some(integer) = optional_integers.pop() {
            if let Some(integer) = integer {
            assert_eq!(integer, cursor);
            cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}
