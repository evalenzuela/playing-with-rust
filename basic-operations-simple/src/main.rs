/**
 * Basic Operations
 * Fix 2 integer numbers
 * save each result on a variable
 * print result of each operation
 */

fn main() {
    let number1: i32 = 4;
    let number2: i32 = 10;
    let sum_result: i32 = number1 + number2;
    let sub_result: i32 = number1 - number2;
    let mul_result: i32 = number1 * number2;
    let div_result: i32 = number1 / number2;

    println!("Sum Result: {}", sum_result);
    println!("Sub Result: {}", sub_result);
    println!("Mul Result: {}", mul_result);
    println!("Div Result: {}", div_result);
}
