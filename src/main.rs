use itertools::Itertools;

fn main() {
    let v = vec![1, 2, 3];
    for w in v.windows(3) {
        let (prev, current, next) = (w[0], w[1], w[2]);
        println!("{} {} {}", prev, current, next);
    }

    for (prev, next) in v.iter().tuple_windows() {
        println!("{} {}", prev, next);
    }

    let v = vec![1, 2, 3, 4, 5];
    let b1 = v.windows(2).all(|a| a[0] < a[1]);
    let b2 = v.iter().tuple_windows().all(|(p, n)| p < n);
    // returns true
    println!("{}", b1);
    println!("{}", b2);

    let v = vec![1, 2, 4, 3, 5];
    let b1 = v.windows(2).all(|a| a[0] < a[1]);
    let b2= v.iter().tuple_windows().all(|(p, n)| p < n);
    // returns false
    println!("{}", b1);
    println!("{}", b2);
}