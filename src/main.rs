use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;

fn cast_the_die() -> u8 {
//    println!("The die is cast");
    let mut rng = rand::thread_rng();
    rng.gen_range(1..7)
}

fn main() {
    // 1秒待機してから2つのサイコロを振り、両方が終わるのを待って
    // それらの目の合計を返す
    thread::sleep(Duration::from_secs(1));
    let h1 = thread::spawn(|| {cast_the_die()});
    let h2 = thread::spawn(|| {cast_the_die()});
    
    let d1 = h1.join().expect("エラー終了");
    let d2 = h2.join().expect("エラー終了");
    println!("{}", d1 + d2);

    // 2つのサイコロを同時に振り、それぞれの結果を同時アクセスが可能
    // な参照に格納し、その値を結果として返す
    let stored = Arc::new(Mutex::new(Vec::<u8>::new()));
    let mut handles = vec![];
    for _ in 0..2 {
        let stored = Arc::clone(&stored);
        let handle = thread::spawn(move || {
            stored.lock().unwrap().push(cast_the_die());
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{:?}", stored);

    // 3つのサイコロを同時に振り、それぞれの結果を同時アクセスが可能
    // な参照に格納し、その値を結果として返す
    let stored = Arc::new(Mutex::new(Vec::<u8>::new()));
    let mut handles = vec![];
    for _ in 0..3 {
        let stored = Arc::clone(&stored);
        let handle = thread::spawn(move || {
            stored.lock().unwrap().push(cast_the_die());
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{:?}", stored);

    // 100個のサイコロを同時に振り、6の目の合計数を同時アクセスが可能
    // な参照に格納し、その値を結果として返す
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            if cast_the_die() == 6 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{:?}", counter);

    // 100個のサイコロを同時に振り、それぞれの前に1秒間待機し、それら
    // の合計を(同時参照を使わずに)返す
    let istart = Instant::now();

    let mut handles = vec![];
    for _ in 0..100 {
        let handle = thread::spawn(|| {
            thread::sleep(Duration::from_secs(1));
            cast_the_die()
        });
        handles.push(handle);
    }

    let sum: u16 = handles.into_iter()
        .map(|h| h.join().unwrap() as u16)
        .sum();

    let istop = Instant::now();

    println!("{:?} {}", istop.duration_since(istart), sum);
}