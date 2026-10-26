fn main() {
    open_store("Bangalore");
    bake(15, "oregano");
    swim_in_profit();
    open_store("Bengal");
}

fn open_store(neighborhood: &str) {
    println!("Opening my Pizza Store in {neighborhood}");
}

fn bake(num: i32, topping: &str) {
    println!("Baking {num} {topping} pizza");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}