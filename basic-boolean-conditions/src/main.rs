/**
 * set a couple variables which result of true or false
 * to print some messages
 */

fn main() {
    let number1: usize = 4;
    let number2: usize = 8;

    let bigger_than: bool = number1 > number2;

    if bigger_than {
        println!("{} is bigger than {}", number1, number2);
    } else {
        println!("{} is not bigger than {}", number1, number2);
    }
}
