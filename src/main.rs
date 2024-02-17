fn main() {
    let stream = vec!["a".to_string(), "b".to_string(), "c".to_string()]
        .into_iter()
        .cycle()
        .take(10)
        .chain(
            (0..10).map(|i| format!("{}", i))
        )
        .chain(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        .collect::<Vec<_>>();

    println!("{:?}", stream);
}