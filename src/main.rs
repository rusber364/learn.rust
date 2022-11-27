fn main() {
    let nick_name: &str = "rusber364";
    let mut age: i8 = 31;

    println!("user with nickname -> {}, age: {}", nick_name, age);

    age = 32;

    println!("user with nickname -> {}, age: {}", nick_name, age);

    const ONE_DAY_SECONDS: i32 = 24 * 60 * 60 * 60;

    println!("One day seconds -> {}", ONE_DAY_SECONDS);

    // Shadowing variables

    let nick_name: &str = "ruslan";

    {
        let nick_name: &str = "rusber364";
        println!("user with nickname -> shadowing:{}", nick_name);
    }

    println!("user with nickname -> {}", nick_name);
}
