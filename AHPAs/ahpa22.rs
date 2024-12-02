#[allow(non_snake_case)]
#[allow(unused_variables)]
 

fn main() {

    let mut string_1 = String::from("Christmas ");
    println!("String: {}, Length: {}", string_1, string_1.len());
    string_1.push('T');
    println!("String: {}, Length: {}", string_1, string_1.len());
    string_1.push('r');
    println!("String: {}, Length: {}", string_1, string_1.len());
    string_1.push('e');
    println!("String: {}, Length: {}", string_1, string_1.len());
    string_1.push('e');
    println!("String: {}, Length: {}", string_1, string_1.len());

    print!("\n");

    let mut string_2 = String::from("Christmas ");
    println!("String: {}, Length: {}", string_2, string_2.len());
    string_2.push_str("Lights");
    println!("String: {}, Length: {}", string_2, string_2.len());


}