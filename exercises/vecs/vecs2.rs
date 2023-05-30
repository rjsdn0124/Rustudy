// vecs2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

// vector 순회하기.
// vector를 iter 형태로 바꿔서 순회할 수 있또록.
// iter은 무조건 iterator 객체로 변경해줌. 얘네는 무조건 mut인듯!!! 앞의 변수가 mut이면 iter_mut 사용하는거고 그냥이면 그냥 iter
// 1. iter_mut() 벡터 원소들을 mut로 바꿔주고 해당 주소값에 있던 element를 다시 값을 덮어 씌우는 느낌? 그래서 mut이 필요한듯! 그대로 주소값을 써용.
// 2. iter().map() 인자를 사용할 때는 |element|형식으로 사용하네요. 여기선 element를 인자로 받으니까 주소값이 초기화되고 다시 return해서 shadowing 같은 느낌으로 사용하는거 같아요.
// 보다는 map은 |element|를 통해서 주소값인 iter 원소들을 그냥 참조값으로 받아오는 거 같아용! 그래서 mut니까 그냥 그 위에 다시 덮어쓰는거고.

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        // v주소값을 넘겨주니까 v를 그대로 쓸 수 있는거지. iter는 새로운 객체를 만드는거니까. v에 덮어쓰지 않아요!!
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
