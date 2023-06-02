// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

// 클리피가 잠재적 오류가 될 수 있는 그런 개발자 실수를 잘 잡아주네.
// 같은 기능을 하는 더 좋은 코드 추천이 개꿀이네. swap 함수를 쓰는게 좋아요! 이런거도 해주네
// 항상 실행은 되는데 오류가 될 코드도 실행 전에 panic 해주넹
// ㄹㅇ eslint

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
