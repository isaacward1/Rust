#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug)]

struct UsfStudent {
   first: String,
   last: String,
   u_num: i32,
   years: i32,
   gpa: f64
}

#[derive(Debug)]
#[allow(dead_code)]
struct Status(i32, i32, i32);

fn main() {

   let student1 = UsfStudent {
      first: String::from("Tim"),
      last: String::from("Johnson"),
      u_num: 2387561,
      years: 3,
      gpa: 2.7
  };

  let student2 = UsfStudent {
   first: String::from("Lars"),
   last: String::from("Nelson"),
   u_num: 9935213,
   years: student1.years,
   gpa: student1.gpa
};

println!("student2 = {:?}", student2);

dbg!(&student2);

let tim_johnson = Status(127, 80, 47);
let lars_nelson = Status(150, 77, 73);

println!("tim_johnson = {:?}", tim_johnson);
println!("lars_nelson  = {:?}", lars_nelson);

}