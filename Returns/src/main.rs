fn main() {
    let result = square(5);
    println!("The square of 5 is {result}");

    let result: i32 = square(8);
    println!("The square of 8 is {result}");

}

fn square(number: i32) -> i32 {
    return number * number;
}
