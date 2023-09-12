fn main() {
    let x = plus_three(4);
    println!("{}", x);
}

fn plus_three(x: i32) -> i32 {
    // add 3 to x.
    x + 3
}
