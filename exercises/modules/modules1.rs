// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

// pub 키워드로 public 메소드나 클래스를 만들 수 있음! mod는 main이랑 같은 스코프니까 사용 가능한거! 
// make_sausage는 main이랑 다른 스코프 내에 있으니 pub 키워드로 호출할 수있도록 바꿔줘야함!

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
