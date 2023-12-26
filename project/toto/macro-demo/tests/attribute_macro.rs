// tests/attribute_macro.rs

use macro_demo::*;

// macro converts struct S to struct H
#[my_custom_attribute]
struct S {
    toto: String,
}

#[test]
fn test_macro() {
    // due to macro we have struct H in scope
    let demo = H {
        test: "t".to_string(),
    };
}
