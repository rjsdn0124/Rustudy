// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// Some 처럼 unpack된 객체를 참조하려면 &가 아닌 ref를 써야한대~. 사실 자세히는 잘 모르겠다. 그냥 외워야겠다.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
