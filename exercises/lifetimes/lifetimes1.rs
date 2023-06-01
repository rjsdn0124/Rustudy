// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk
// of going out of scope before it is used. Remember, references are borrows
// and do not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a hint.

// as_str 함수로 &str을 만들 수 있넹~
// 'a 모르겠다. 하면서 공부해야겟다. 'a가 가장 위에 있을 때까지는 살아간단 말 같은디
// 아 인자로 둘 다 같은 자료형이 들어오면 둘 중 어떤 놈이 반환이 될지 체크가 안됨.
// 내생각인데 그래서 둘 중에 하나가 반환은 됐는데 하나가 인자로 들어와서 힙에 저장이 되었는데 해제가 안되고 계속 가지고 있으니까 안됨.
// 그래서 'a 어노테이션으로 얘네 오래 살아갈거야~ 라고 말해주는 느낌?
// 적어보니까 아니네 뭐냐..?

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
