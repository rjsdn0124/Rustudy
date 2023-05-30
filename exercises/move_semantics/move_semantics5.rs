// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

// 가변 참조자를 2개 선언 못해용. 한번 선언하고 다음 선언 전에 모두 사용해야해요. 다음 선언 이후에 이전 선언한 변수를 사용하면 참조 오류라고 하네요!
// 개발자가 할 수 있는 실수를 막기 위한 장치인 것 같슴당~

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
