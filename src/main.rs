fn main() {
    let fib = fibonacci(10);

    println!("fibonacci:10 -> {fib}");
    convertor_temperature(79.0, 'f');
    convertor_temperature(32.0, 'c');
    range_numbers(10, 15);
    range_numbers(4, 8)
}

fn range_numbers(from: i32, to: i32) {
    println!("range {from}...{to}: ");
    for n in from..to + 1 {
        println!("-> {n}");
    }
}

fn convertor_temperature(temperature: f64, format: char) {
    if format == 'c' {
        let fahrenheit: f64 = celsius_to_fahrenheit(temperature);
        println!("convertor_temperature: {temperature:.2}°C -> {fahrenheit:.2}°F");
    }

    if format == 'f' {
        let celsius: f64 = fahrenheit_to_celsius(temperature);
        println!("convertor_temperature: {temperature:.2}°F -> {celsius:.2}°C");
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fibonacci(num: u32) -> u32 {
    let mut data: [u32; 2] = [1, 1];
    let mut idx: u32 = 2;

    while idx < num {
        let [first, second] = data;

        let next_first: u32 = second;
        let next_second: u32 = first + second;

        data[0] = next_first;
        data[1] = next_second;

        idx += 1;
    }

    data[1]
}
