fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    let condition = true;
    let number = if condition { 5 } else { 3 }; // expression
    println!("the value of numbers is {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");
    counting_up();
    with_while();
    with_for();
}

fn counting_up() {
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("end count: {count}");
}

fn with_while() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("lift off");
}

fn with_for() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is {element}");
    }
}
