fn main() {
    let num: u128 = 30;
    let result: u128 = factorial(num);

    println!("factorial of {num} is: {result}");
}

fn factorial(n: u128) -> u128 {
    if n == 1 {
        return 1;
    }

    n * factorial(n - 1)
}
