// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!

// 오 다른건 그 전에 공부해서 알겠는데 into는 let a:String = "adfa".into()로 하면 앞에서 선언된 자료형으로 변환되어서 들어가네!
// 여기선 함수 안에 인자로 들어가니까 let arg: &str = ~~ 로 되어있으니까 str로 변환되어서 들어가네!
// 원리는 앞의 자료형에 from이 구현되어있으면 &str::from()함수를 자동으로 호출하네! 그래서 그렇게 원하는 형식으로 만들어주는구만!
// let arg:String = "asdf".into() == let arg:String = String::From("asdf") 입니다!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
