pub enum Transmission {
    Auto,
    Mechanical,
}

pub struct CarModel {
    pub name: String,
    pub color: (u8, u8, u8),
    pub transmission: Transmission,
}

pub enum Cars {
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

    pub fn get_info(&self) {
        let car_name = self.get_model();
        let car_color = self.get_color();
        let car_transmission = self.get_transmission();

        println!("Car model:");
        println!("*****************");
        println!("Name -> {car_name}");
        println!("Color -> {car_color:?}");
        println!("Transmission -> {car_transmission}");
        println!("*****************");
    }
}
