// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

// 함수 선언은 fn으로 선언한다잉!
// ;이 붙으면 함수 안 그냥 코드들. ;가 안 붙는다면 return문이다!
// print문 안에 함수도 들어갈 수 있고, String::new를 사용하면 String형인데 그냥 문장을 쓰면 &str이다!
// 그냥 문장을 그대로 사용해서 그런지 static 선언을 해줘야된다고 한다!

fn main() {
    call_me();
    println!("{}", return_call());
}

fn call_me(){
    println!("hello world!");
}
fn return_call()->&'static str{
    "hello world!"
}