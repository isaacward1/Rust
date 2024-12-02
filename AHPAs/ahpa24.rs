

fn print_stu(a:&str, b:&i32) {
   println!("Student {:4} has a score of {}.", a, b);
}

fn round(a:&str, b:&i32) {
   println!("Student {:4} has a score of {}.", a, b + 5);
}



fn main() {
   let list = vec![("Tim", 79), ("Mary", 92), ("Tom", 83), ("Jane", 87), ("Ann", 75)];

      for (a, b) in &list {  
      print_stu(a, b);
      }
      print!("\n");
      for (a, b) in &list {  
         round(a, b);
         }
   }