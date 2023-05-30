// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// (a,v1,v1) 하면 v1이 바로 없어지네. 앞에게 호출되면 바로 메모리를 놓아주넹.
// 배열을 vec으로 하는 법!
// 1. 하나씩 push해주기.
// 2. 배열변수.to_vec()해주기.
// 3. vec![a[0],a[1],a[2]] 매크로에 이런 식으로 넣어주기. vec 매크로는 정해진 규칙으로 넣어줘야지 되기때문에 저런 방식만 된다고 한다. vec![a]는 안됨~!
// 매크로란 println! 처럼 Rust 내의 내장 함수인듯하다!

fn array_and_vec() -> ([i32; 4], Vec<i32>, Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let mut v1 = Vec::<i32>::new();// TODO: declare your vector here with the macro for vectors
    for i in a {v1.push(i);}
    let v2 = a.to_vec();
    (a, v1, v2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v1, v2) = array_and_vec();
        assert_eq!(a, v1[..]);
        assert_eq!(a, v2[..]);
    }
}
