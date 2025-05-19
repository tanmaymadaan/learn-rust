fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    another_function3(5, 'h');
    another_function4(5);
    add_five(2);
}

fn another_function() {
    println!("Another function");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn another_function3(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function4(value: i32) {
    let y = {
        // let x = 3;
        value + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn add_five(value: i32) {
    let sum = value + five();
    println!("The sum after adding 5 is {sum}");
}