struct User {
    name: String,
    age: i8,
    developer: bool,
}

fn create_user(name: &str, age: i8, developer: bool) -> User {
    User {
        name: String::from(name),
        age,
        developer,
    }
}

fn main() {
    let mut user1 = create_user("rusber364", 32, true);

    user1.name.push_str("__*123*_");

    println!("Name -> {}", user1.name);
    println!("Age -> {}", user1.age);
    println!("Developer -> {}", user1.developer);
}
