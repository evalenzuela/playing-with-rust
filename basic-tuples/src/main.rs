fn main() {
    let tuples: (u32, &str, f32) = (100, "1", 20.50);
    println!("1: {}", tuples.0);
    println!("2: {}", tuples.1);
    println!("3: {}", tuples.2);
}
