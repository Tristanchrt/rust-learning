fn toto() {
    let v1 = vec![1, 2, 3, 4];

    let v2: Vec<_> = v1
        .iter()
        .filter(|&num| *num % 2 == 0)
        .map(|x| x + 1)
        .collect();

    for x in &v2 {
        println!("{}", x)
    }
}
