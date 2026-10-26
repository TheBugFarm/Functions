fn apply_to_jobs(num: i32, title: &str) {
    println!("i'm applying to {num} {title} jobs");
}

fn is_even(num: i32) -> bool{
    num % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains('a'), text.contains('z'))
}
fn main() {
   apply_to_jobs(35, "Rust Developer"); 
   
   is_even(8);
   is_even(9);

   println!("{:?}", alphabets("aardvark"));
   println!("{:?}", alphabets("zoology"));
   println!("{:?}", alphabets("zebra"));

}

