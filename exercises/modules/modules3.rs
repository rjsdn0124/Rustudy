// modules3.rs
// You can use the 'use' keyword to bring module paths from modules from anywhere
// and especially from the Rust standard library into your scope.
// Bring SystemTime and UNIX_EPOCH
// from the std::time module. Bonus style points if you can do it with one line!
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a hint.

// module만 use할 수 있넹. systemTime이 구조체라 그 이후 함수들은 use가 안됑. module이 아닐때까지 use 참조를 할 수 있넹
// 상수도 할 수 있넹

// TODO: Complete this use statement
use std::time;
// 위에거랑 아래거 둘 다 사용 가능합니다. 위에는 한 줄. 밑에는 사용할 내용만.
// use std::time::SystemTime
// use std::time::UNIX_EPOCH

fn main() {
    match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
