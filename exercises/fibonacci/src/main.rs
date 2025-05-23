fn main() {
    // fibonacci sequence
    let n = 10;
    let mut a = 0;
    let mut b = 1;
    let mut c;

    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }

    println!("the {n}th fibonacci number using loop is {a}");

    // fibonacci sequence using recursion
    let n = 10;
    let result = fibonacci(n);
    println!("the {n}th fibonacci number using recursion is {result}");
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
