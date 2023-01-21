fn main() {
    let nick_name = String::from("rusber");
    let nick_name_len = nick_name.len();

    println!("my nickname is -> {}, len -> {nick_name_len}", nick_name);

    let (update_nick_name, update_len) = add_str_to_nickname(nick_name, "364");

    println!("my nickname is -> {update_nick_name}, len -> {update_len}, after update");
}

fn add_str_to_nickname(mut nick_name: String, new_str: &str) -> (String, usize) {
    nick_name.push_str(new_str);
    let nick_name_len = nick_name.len();

    (nick_name, nick_name_len)
}
