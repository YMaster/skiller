use std::fmt::{Display, Formatter, Result};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}
impl Display for City {
    fn fmt(&self, format: &mut Formatter) -> Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            format,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.3242432,
            lon: -6.242352,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ].iter(){
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 12,
            green: 23,
            blue: 33,
        },
        Color {
            red: 99,
            green: 99,
            blue: 99,
        },
    ]
    .iter()
    {
        println!("{:?}", *color);
    }

    // Activity
    struct ColorShow {
        red: u8,
        green: u8,
        blue: u8,
    }
    fn get_hax(color: u8) -> String {
        if color > 15 {
            format!("{:X}", color)
        } else {
            format!("0{:X}", color)
        }
    }
    impl Display for ColorShow {
        fn fmt(&self, format: &mut Formatter) -> Result {
            let mut out_res = String::from("");
            out_res.push_str(&get_hax(self.red));
            out_res.push_str(&get_hax(self.green));
            out_res.push_str(&get_hax(self.blue));
            write!(
                format,
                "RGB ({}, {}, {}) 0x{}",
                self.red, self.green, self.blue, out_res
            )
        }
    }

    let color_show = ColorShow {
        red: 5,
        green: 56,
        blue: 255,
    };
    println!("{}", color_show);
}
