#[allow(non_snake_case)]
#[allow(unused_variables)]
#[derive(Debug)]

struct Player {
   stu_name: String,
   game_name: String,
   age: i32,
   level: i32,
}

fn main() {

let player_data = vec![
"Bob Johnson:Master Sergeant:21:7", 
"Rebecca Hold:Slay:19:4", 
"John Majors:Murader:20:6",
"Ann Jenkens:Force:22:9"];

let mut struct_list = vec![];

for x in player_data {
   let parts: Vec<&str> = x.split(':').collect();

   let cur_player = Player {
      stu_name: parts[0].to_string(),
      game_name: parts[1].to_string(),
      age: parts[2].parse().expect(""),
      level: parts[3].parse().expect(""),
  };
  struct_list.push(cur_player);
}

for b in struct_list {
   println!("{:?}", b);
}

}