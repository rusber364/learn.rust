mod enginery;
use enginery::{CarModel, Cars, Transmission};

fn main() {
    let volkswagen = Cars::Volkswagen(CarModel {
        name: String::from("Gold"),
        color: (255, 255, 255),
        transmission: Transmission::Auto,
    });

    let ford = Cars::Ford(CarModel {
        name: String::from("Mustang"),
        color: (255, 255, 255),
        transmission: Transmission::Mechanical,
    });

    let honda = Cars::Honda(String::from("Civic"));
    let toyota = Cars::Toyota(String::from("Corolla"));
    let nissan = Cars::Nissan(String::from("Skyline"));
    let bmw = Cars::Bmw {
        name: String::from("X6"),
        color: (127, 127, 127),
    };
    let mercedes_benz = Cars::MercedesBenz {
        name: String::from("Vito"),
    };

    volkswagen.get_info();
    ford.get_info();
    honda.get_info();
    toyota.get_info();
    nissan.get_info();
    bmw.get_info();
    mercedes_benz.get_info();
}
