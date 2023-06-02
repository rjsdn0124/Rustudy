// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.

// This program spawns multiple threads that each run for at least 250ms,
// and each thread returns how much time they took to complete.
// The program should wait until all the spawned threads have finished and
// should collect their return values into a vector.

// thread는 joinhandle enum이랭 그래서 .join함수가 await해주는 함수 느낌?
// 그러고 thread니까 실행 순서를 보장하지 안ㅇ흠!

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(
            // 쓰레드 생성
            // move가 thread안에서든 어디든 들어가면 참조값을 빌리는게 아닌 그대로 값 바로 갖다 박아서 소유권을 가져오는 키워드!
            // thread는 다른 쓰레드의 변수의 주소값에 접근하기 어려우니 그냥 move를 통해서 값을 그대로 갖다박는듯!
            // 쓴 변수는 바로 drop된다!
            thread::spawn( move || {
            // 시간을 재기 시작
            let start = Instant::now();
            // 쓰레드는 백그라운드로 실행되니 백에서 재워
            thread::sleep(Duration::from_millis(250));
            // move로 가져온 변수 사용!
            println!("thread {} is complete", i);
            // 시간 멈추고 차이를 milli sec로 반환!
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
