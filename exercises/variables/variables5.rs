// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

// 변수 이름은 같게 하고 싶지만 자료형이 바뀌어야 할 때 우리는 shadowing을 쓴대요
// 변수를 이름과 주소값으로 할당하는데 그 주소값을 덮어쓰는 느낌? scope마다 달라 근데. 
// inner scope일 때는 그냥 새로운 변수 느낌으로 새로 생성되어 들어간다!

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number:i32 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
