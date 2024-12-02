enum StudentYears {
   Freshman(String),  
   Sophomore(f32),   
   Junior(String),   
   Senior(String)
}

impl StudentYears {
   fn get_info(&self) {
   print!("Student info: ");
   match self {
      StudentYears::Freshman(name) => println!("Freshman({})", name), 
      StudentYears::Sophomore(gpa) => println!("Sophomore({})", gpa),
      StudentYears::Junior(internship) => println!("Junior({})", internship),
      StudentYears::Senior(job) => println!("Senior({})", job)
   }
   }
}

fn main() {
   let freshman = StudentYears::Freshman(String::from("Tim Johnson"));
   let sophomore = StudentYears::Sophomore(3.3);
   let junior = StudentYears::Junior(String::from("false"));
   let senior = StudentYears::Senior(String::from("true"));

   freshman.get_info();
   sophomore.get_info();
   junior.get_info();
   senior.get_info();
}
