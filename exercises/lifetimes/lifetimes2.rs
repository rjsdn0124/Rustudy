// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.

// Reference : https://blog.naver.com/PostView.nhn?blogId=sssang97&logNo=221594974122
// 깨달았다!
// '? 어노테이션을 달때 함수 이름이나 객체 이름에도 제너릭처럼 달아주는데, 
// 적어도 이 함수랑 lifetime을 같이 가진다고 걱정말라고 컴파일러에게 설명해주는거라고 한다!
// ---------------------------------중ㅇ요----------------------------------------
// 컴파일러의 걱정은 그거지 x,y가 있는데 결과로는 실제 runtime이 되어봐야 아는건데
// 개발자 입장에서는 당연히 결과로 예상한 부분을 반환하겠거니하고 남은 하나의 lifetime을 신경쓰지 않게되는데,
// 혹시 x로 에상했다가 Y가 들어오면 메모리를 계속 날려버리는 러스트 입장에서는 너무 미안한거지.
// 그래서 너가 잘하면 내가 실수를 안하자나로 가스라이팅 계속 해서 주석으로 실수 하지마라. 실수하지마라. 라고 하는거같다!
// 그래서 주석을 달아주면 일단 나는 이 함수 내에서는 같은 스코프에서 절대 변수 안 버리니
// 혹시라도 변수가 없어지면 밖에서 없어진거야!! 라고 하기 위해서!!!!
// 와우 깨달았따!!!!
// 혹시 모를 개발자의 실수를 위해 진짜 그냥 annotation같은 거같다! 와우 미친사람들.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}
