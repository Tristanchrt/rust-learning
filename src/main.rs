macro_rules! my_macro {
    () => {
        println!("This is a declarative macro");
    };
}

fn main() {
    my_macro!(); // This will be replaced with println!("This is a declarative macro");
}
