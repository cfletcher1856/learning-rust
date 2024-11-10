// Single line comment

/* Block comments */

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;

    // Scaler data types
    // Integers
    // Floating-point numbers
    // Booleans
    // Characters

    let sum = my_function(11, 22);
    println!("The sum is: {}", sum);

    my_loops();
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let sum = x + y;

    return sum;
}

fn my_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The value is: {}", result);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("The value is: {}", number);
    }
}
