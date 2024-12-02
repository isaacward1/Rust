use std::rc::Rc;

fn main() {
    let name = String::from("Bob Johnson");
    let name_ptr = &name;   
    println!("name_ptr = {}", name_ptr);

    
    let name_remote = Box::new(name);
    println!("name_remote = {}", *name_remote);
    
    let name2 = Rc::new(String::from("Ann Majors"));
    let name2_c1 = name2.clone();
    let name2_c2 = name2.clone();
    println!("{} {} {}", name2, name2_c1, name2_c2);
}