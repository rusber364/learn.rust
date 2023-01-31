#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn size(&self, info: &str) -> u32 {
        let s = self.height * self.width;
        println!("{info}: {s}");

        s
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(rect1.size("rusber364"));
    dbg!(rect1);
}
