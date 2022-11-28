fn main() {
    // scalar integer
    // i8   -->  -128...127
    // u8   -->     0...255
    let integer: i8 = 127;
    println!("{}", integer);
    let unsigned_integer: u8 = 255;
    println!("{}", unsigned_integer);

    // i16  -->  -32_768...32_767
    // u16  -->        0...65_536
    let integer: i16 = 32_767;
    println!("{}", integer);
    let unsigned_integer: u16 = 65_535;
    println!("{}", unsigned_integer);

    // i32  -->  -2_147_483_648...2_147_483_647
    // u32  -->               0...4_294_967_296
    let integer: i32 = 2_147_483_647;
    println!("{}", integer);
    let unsigned_integer: u32 = 4_294_967_295;
    println!("{}", unsigned_integer);

    // i64  -->  -9_223_372_036_854_775_808...9_223_372_036_854_775_807
    // u64  -->                           0...18_446_744_073_709_551_615
    let integer: i64 = 9_223_372_036_854_775_807;
    println!("{}", integer);
    let unsigned_integer: u64 = 18_446_744_073_709_551_615;
    println!("{}", unsigned_integer);

    // i128  -->  -170_141_183_460_469_231_731_687_303_715_884_105_728...170_141_183_460_469_231_731_687_303_715_884_105_727
    // u128  -->                                                     0...340_282_366_920_938_463_463_374_607_431_768_211_455
    let integer: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    println!("{}", integer);
    let unsigned_integer: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    println!("{}", unsigned_integer);

    // isize  -->  i32, i64
    // usize  -->  u32, u64
    let integer: isize = 2_147_483_647;
    println!("{}", integer);
    let unsigned_integer: usize = 4_294_967_295;
    println!("{}", unsigned_integer);

    // scalar floating
    let floating64: f64 = 1.797_693_134_862_315_7e308;
    println!("{floating64}");
    let floating32: f32 = 3.402_823_5e38;
    println!("{floating32}");

    // scalar bool
    let is_auth: bool = true;
    println!("{is_auth}");
    let is_auth: bool = false;
    println!("{is_auth}");

    // scalar char
    let w: char = 'w';
    println!("{}", w);
    let r: char = 'r';
    println!("{}", r);

    // tuple
    let (user_nick, user_age, user_is_programming): (&str, u8, bool) = ("rusber364", 32, true);
    println!("User nick -> {user_nick}, age -> {user_age}, {user_is_programming}");

    // array
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("January -> {}", months[0]);
    println!("December -> {}", months[11]);
}
