
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        match self {
            Message::Write(data) => println!("data of Write variant: {}", data),
            Message::Move { x, y } => println!("Move variant: x={}, y={}", x, y),
            Message::Quit(data) => println!("Move variant: x={}, y={}", data),
            _ => println!("Variant does not contain data to display."),

        }
    }
}

enum Option<T> {
    None,
    Some(T),
}

fn get_value(condition: bool) -> Option<i32> {
    if condition {
        Option::Some(42)
    } else {
        Option::None
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska(String)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        _ => 10
    }
}

fn main() {

    let m = Message::Write(String::from("hello"));
    m.call();
    
    let pomme = MoveMessage {
        x: 10,
        y: 20,
    };

    let a = match pomme {
        MoveMessage { x, y } => Message::Move { x, y },
    };
    a.call();

    let result = get_value(true);

    match result {
        Option::Some(value) => println!("Value is {}", value),
        Option::None => println!("No value present"),
    }

    let toto = Coin::Quarter(UsState::Alaska(String::from("hello")));
    value_in_cents(toto);
    
}
