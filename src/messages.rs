pub fn print_too_small() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Too small! {result}");
}

pub fn print_too_big() {
    let x = {
        
        let x = 10;
        let _= x + 10;
        let y = {
            x * x
        };
        y
    };

    println!("Too big! {x}");
}

pub fn print_you_win() {
    println!("You win!");
}
