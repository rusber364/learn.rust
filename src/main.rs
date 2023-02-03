enum Transmission {
    Auto,
    Mechanical,
}

struct CarModel {
    name: String,
    color: (u8, u8, u8),
    transmission: Transmission,
}

enum Cars {
    Volkswagen(CarModel),
    Ford(CarModel),
    Honda(String),
    Nissan(String),
    Toyota(String),
    MercedesBenz { name: String },
    Bmw { name: String, color: (u8, u8, u8) },
}

impl Cars {
    fn get_model(&self) -> &str {
        match self {
            Self::Volkswagen(model) => &model.name,
            Self::Ford(model) => &model.name,
            Self::MercedesBenz { name } => name,
            Self::Bmw { name, color: _ } => name,
            Self::Honda(name) | Self::Nissan(name) | Self::Toyota(name) => name,
        }
    }

    fn get_color(&self) -> (u8, u8, u8) {
        match self {
            Self::Volkswagen(model) => model.color,
            Self::Ford(model) => model.color,
            Self::Bmw { name: _, color } => *color,
            Self::Honda(_) | Self::Nissan(_) | Self::Toyota(_) | Self::MercedesBenz { .. } => {
                (255, 255, 255)
            }
        }
    }

    fn get_transmission(&self) -> &str {
        match self {
            Self::Ford(model) | Self::Volkswagen(model) => match model.transmission {
                Transmission::Auto => "auto",
                Transmission::Mechanical => "mechanical",
            },
            Self::Honda(_)
            | Self::Nissan(_)
            | Self::Toyota(_)
            | Self::Bmw { .. }
            | Self::MercedesBenz { .. } => "auto",
        }
    }
}

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

    // volkswagen
    info_car(&volkswagen);
    // ford
    info_car(&ford);
    // honda
    info_car(&honda);
    // toyota
    info_car(&toyota);
    // nissan
    info_car(&nissan);
    // bmw
    info_car(&bmw);
    // mercedes_benz
    info_car(&mercedes_benz);
}

fn info_car(car: &Cars) {
    let car_name = car.get_model();
    let car_color = car.get_color();
    let car_transmission = car.get_transmission();

    println!("Car model:");
    println!("*****************");
    println!("Name -> {car_name}");
    println!("Color -> {car_color:?}");
    println!("Transmission -> {car_transmission}");
    println!("*****************");
}
