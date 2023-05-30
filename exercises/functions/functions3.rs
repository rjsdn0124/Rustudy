// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

// 그냥 인자가 없어서 안됨. 별거 없긴해~

fn main() {
    call_me(3);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
