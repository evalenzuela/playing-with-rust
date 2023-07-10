fn main() {
    let number: i8 = evaluat_number(100);
    println!("Result is {}", number);
}

fn evaluat_number(number: i32) -> i8 {
    if number < 0 {
        return -1;
    } else if number < 10 {
        return 0;
    }

    1
}
