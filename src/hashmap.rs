use std::collections::HashMap;


fn main() {

    // Create an empty HashMap that maps strings to integers
    let mut scores = HashMap::new();

    // Insert values into the HashMap
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 80);
    scores.insert(String::from("Charlie"), 90);

    // Access values using their keys
    let alice_score = scores.get("Alice");
    match alice_score {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice's score not found"),
    }

    // Iterate through the HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwrite an existing value
    scores.insert(String::from("Bob"), 85);

    // Check if a key exists and update the value if it does
    let name = "Charlie";
    scores.entry(String::from(name)).or_insert(95);

    // Remove an entry from the HashMap
    scores.remove("Alice");

    // Print the final content of the HashMap
    println!("Updated scores:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    if scores.contains_key("Blue") {
        println!("Yes, the key 'Blue' exists in the HashMap");
    } else {
        println!("No, the key 'Blue' does not exist in the HashMap");
    }
    println!("{:?}", scores);


}
