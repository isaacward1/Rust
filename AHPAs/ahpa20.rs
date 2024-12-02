#[allow(non_snake_case)]
#[allow(unused_variables)]

fn main() {
    
    let compNames = ["Red", "Blue", "Green", "Purple"];
    let mut stuNames = vec!["Bob", "Jane", "Ann", "Tony"];

    let mut x = 0;
    while x < stuNames.len() {
        println!("Student {} has been assigned computer {}", stuNames[x], compNames[x]);
        x += 1;
    }

    stuNames.remove(2);
    println!("\n");

    x = 0;
    while x < stuNames.len() {
        println!("Student {} has been assigned computer {}", stuNames[x], compNames[x]);
        x += 1;
    }


}
