#[allow(non_snake_case)]
#[allow(unused_variables)]

fn main() {
    
    let lastName1 = "Johnson";
    let lastName2 = "Andrews";
    
    assert!(lastName1 == "Johnson" && lastName2 == "Andrews");
    println!("Assertion is correct");

    let firstName1 = "Ann";
    let firstName2 = "Joe";
    let oldEnough1:bool = true;
    let oldEnough2:bool = false;

    assert!(firstName1 == "Ann" && oldEnough2 == false);
    println!("Assertion is correct");
}
