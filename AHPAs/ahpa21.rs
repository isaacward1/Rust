#[allow(non_snake_case)]
#[allow(unused_variables)]

fn square_cube_even_odd(num: i32) {
    let square = num * num;
    let cube = num * num * num;
    let mut square_eo = "Odd";
    let mut cube_eo = "Odd";

if square % 2 == 0 { 
    square_eo = "Even";
}
if cube % 2 == 0 { 
    cube_eo = "Even";
}
    println!("{}\t{}\t{}\t{}\t{}", num, square, square_eo, cube, cube_eo);
}

fn main() {
println!("Index Squared Even/Odd Cubed Even/Odd");
println!("----- ------- -------- ----- --------");
    for x in 1..11 {
        square_cube_even_odd(x);
    }
}