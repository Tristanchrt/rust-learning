fn test() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut only_borrows = || {
        list.push(7);
        println!("From closure: {:?}", list);
    };

    only_borrows();
    println!("After calling closure: {:?}", list);
}