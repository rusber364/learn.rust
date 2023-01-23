fn main() {
    let mut nick_name = String::from("rusber");
    let nick_name_len = nick_name.len();

    println!("my nickname is -> {nick_name}, len -> {nick_name_len}");

    let nick_name_len = add_str_to_nickname(&mut nick_name, "364");

    println!("my nickname is -> {nick_name}, len -> {nick_name_len}, after update");
}

fn add_str_to_nickname(nick_name: &mut String, new_str: &str) -> usize {
    nick_name.push_str(new_str);

    nick_name.len()
}
