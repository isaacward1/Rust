#[allow(non_snake_case)]
#[allow(unused_variables)]

fn main() {
    
    let cBowlBalls = ["red", "green", "yellow", "blue", "purple"];

    println!("Children's Bowling Balls");
    println!("{}", cBowlBalls[0]);
    println!("{}", cBowlBalls[1]);
    println!("{}", cBowlBalls[2]);
    println!("{}", cBowlBalls[3]);
    println!("{}\n", cBowlBalls[4]);
    
    let mut ABowlBalls = vec!["gold", "silver", "aqua", "pink", "brown"];
    ABowlBalls.push("lavendar");
    
    println!("Adult's Bowling Balls");
    println!("{}", ABowlBalls[0]);
    println!("{}", ABowlBalls[1]);
    println!("{}", ABowlBalls[2]);
    println!("{}", ABowlBalls[3]);
    println!("{}", ABowlBalls[4]);
    println!("{}", ABowlBalls[5]);

}
