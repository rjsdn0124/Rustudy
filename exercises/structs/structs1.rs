// structs1.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a hint.

// 오 구조체가 잘 되어있네 classic struct는 늘 아는 구조체로 쓰면 되고, Tuple struct는 함수에 인자 받는거처럼 사용하는데 tuple 형태로 반환하는거.
// unit like struct는 그냥 구조체의 변수 명이 변수의 값이 되어 그냥 const define 된 string을 사용하는 느낌!

struct ColorClassicStruct {
    red:i32,green:i32,blue:i32
}

struct ColorTupleStruct(i32,i32,i32);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct{red:0,green:255,blue:0};

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0,255,0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
