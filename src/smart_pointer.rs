// ### **Smart pointer :**

// ### **1. `Box<T>`:**

// In this example, a recursive data structure (**`LinkedList`**) is created using **`Box<T>`** to manage memory on the heap.

// ```rust
// rustCopy code
// // Define a recursive LinkedList structure using Box
// enum LinkedList {
//     Node(i32, Box<LinkedList>),
//     Nil,
// }

// use LinkedList::*;

// fn main() {
//     let list = Node(1, Box::new(Node(2, Box::new(Node(3, Box::new(Nil))))));

//     match list {
//         Node(value, next) => {
//             println!("Value: {}", value);
//             if let Node(_, _) = *next {
//                 println!("Has next node.");
//             }
//         }
//         Nil => println!("End of list."),
//     }
// }

// ```

// ### **2. `Rc<T>`:**

// Illustrating multiple ownership using **`Rc<T>`** to share data across different parts of the program.

// ```rust
// rustCopy code
// use std::rc::Rc;

// struct Person {
//     name: String,
//     age: u8,
// }

// fn main() {
//     let shared_person = Rc::new(Person {
//         name: "Alice".to_string(),
//         age: 30,
//     });

//     // Clone the reference
//     let alice = Rc::clone(&shared_person);
//     let another_alice = Rc::clone(&shared_person);

//     println!("Name: {}", shared_person.name); // Access shared data

//     // Print reference counts
//     println!("Reference count: {}", Rc::strong_count(&shared_person)); // Outputs 3
// }

// ```

// ### **3. `Arc<T>`:**

// Using **`Arc<T>`** for multithreaded access to shared data:

// ```rust
// rustCopy code
// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let shared_data = Arc::new(Mutex::new(0));

//     let mut handles = vec![];

//     for _ in 0..5 {
//         let data = Arc::clone(&shared_data);

//         let handle = thread::spawn(move || {
//             let mut data = data.lock().unwrap();
//             *data += 1;
//         });

//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Final value: {:?}", shared_data.lock().unwrap());
// }

// ```

// ### **4. `Mutex<T>`:**

// Using **`Mutex<T>`** to safely modify shared data within multiple threads:

// ```rust
// rustCopy code
// use std::sync::{Mutex, Arc};
// use std::thread;

// fn main() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {:?}", *counter.lock().unwrap());
// }

// ```

// ### **5. `RefCell<T>`:**

// Using **`RefCell<T>`** to mutate interior data even with immutable references:

// rustCopy code
// use std::cell::RefCell;

// fn main() {
//     let data = RefCell::new(vec![1, 2, 3]);

//     // Borrowing data mutably
//     let mut borrowed_data = data.borrow_mut();
//     borrowed_data.push(4);
//     drop(borrowed_data); // Explicitly drop the mutable borrow

//     // Borrowing data immutably
//     let borrowed_immutable = data.borrow();
//     println!("Data: {:?}", *borrowed_immutable);
// }

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
