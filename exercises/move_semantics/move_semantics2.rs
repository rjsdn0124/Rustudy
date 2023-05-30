// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`

// 1번 해결법!
// vec0.clone()하면 실행은 되지. vec0도 그대로 남아있어서 호출도 되고 fill_vec에도 들어갈 수 있고 나와서 mut vec1에도 값이 변경될 수 있고. 하지만 expected output에 만족하지 않음!
// fn main() {
//     let vec0 = Vec::new();

//     // Do not move the following line!
//     let mut vec1 = fill_vec(vec0.clone());

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// 2번 해결법
// 주소값으로 접근하기 근데 어차피 함수 안에서 clone을 하니까 원하는 결과값이 나오지 않는다. 결국 vec0을 mut로 바꿔야지 원하는 결과가 나오겠지?
// fn main() {
//     let vec0 = Vec::new();

//     // Do not move the following line!
//     let mut vec1 = fill_vec(&vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// 3번 해결법
// mut인 주소값(가변 참조자. 그냥 &만 사용하면 불변참조자라 읽기만 할 수 있따. 그 변수가 mut이더라도.)을 인자로 줬다.
// 이를 이용해서 vec0이 가리키는 pointer에 push한다. 그러고 다시 반환. 반환할 때 vector로 만들어서 주기때문에 새로운 메모리에 할당되어서 전달된다.
// vec0이랑 반환된 vector의 주소값 둘 다 있는 것! 그래서 이를 vec1로 넣어준다. 둘은 다른 주소값이기 때문에 서로 독립적이다.
// 둘 다 유효한 drop되지 않은 주소값이기 때문에 모두 접근이 가능하다!
// 만약 독립적이지 않고 종속적으로 사용하고 싶다면 함수의 반환값을 가변 주소값으로 변경하여 vec1에 push를 하게된 후 .to_vec()함수를 사용하면 
// vec1에 push한 값이 vec0에도 들어있다!!

fn main() {
    let mut vec0 = Vec::new();

    // Do not move the following line!
    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}
