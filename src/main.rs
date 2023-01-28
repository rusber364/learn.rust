fn main() {
    let mut user_name: String = String::from("rusber364");
    user_name.push_str("_123");

    let slice_user_name: &str = &user_name[6..];
    println!("User name -> {user_name}, slice -> {slice_user_name}");

    let mut list_languages_programming = ["rust", "typescript", "javascript", "web_assembly"];

    list_languages_programming[2] = "javascript_es2022";

    for (i, &lang) in list_languages_programming.iter().enumerate() {
        println!("{i} -> {lang}");
    }
}
