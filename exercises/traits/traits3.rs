// traits3.rs
//
// Your task is to implement the Licensed trait for
// both structures and have them return the same
// information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a hint.

// 인터페이스 같지만 안에서도 선언이 가능하네! 그냥 함수 추가해주는 정도로만 생각해야할듯! not interface!
// trait의 self로 ownership을 관리하는 걸 깨달았다...!

pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
