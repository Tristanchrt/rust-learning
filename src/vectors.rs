enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    
    for i in &mut v {
        *i += 50;
        println!("TOTO : {i}");
    }


    let mut row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    row.push(SpreadsheetCell::Float(30.0));

    match row[0] {
        SpreadsheetCell::Int(value) => println!("TOTO: {}", value), // Match and print the value
        _ => println!("Not an Integer"),

    }

}
