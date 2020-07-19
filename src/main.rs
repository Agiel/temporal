use chrono::prelude::*;
use clap::Clap;
use sunrise::sunrise_sunset;

const SECONDS_PER_DAY: i64 = 60 * 60 * 24;

const LATIN: &str = "3 4 5 6 7 8 9 10 11 12 1 2";
const KANJI: &str = "六 五 四 九 八 七 六 五 四 九 八 七";
const ZODIAC: &str = "卯 辰 巳 午 未 申 酉 戌 亥 子 丑 寅";

// Prints the current temporal hour
#[derive(Clap)]
#[clap(author = "Agiel Negura <agiel.negura@gmail.com")]
struct Opts {
    // Latitude
    latitude: f64,
    // Longitude
    longitude: f64,
    // How the hour should be printed. One of latin, kanji or zodiac.
    #[clap(short, long, default_value = "latin")]
    mode: String,
}

fn main() {
    let opts = Opts::parse();

    let now = Local::now();
    let (sunrise, sunset) = sunrise_sunset(
        opts.latitude,
        opts.longitude,
        now.year(),
        now.month(),
        now.day(),
    );

    let now = now.timestamp();
    let hour = if now >= sunrise && now < sunset {
        (now - sunrise) as f64 / (sunset - sunrise) as f64 * 6.
    } else if now < sunrise {
        let last_sunset = sunset - SECONDS_PER_DAY;
        (now - last_sunset) as f64 / (sunrise - last_sunset) as f64 * 6. + 6.
    } else {
        let next_sunrise = sunrise + SECONDS_PER_DAY;
        (now - sunset) as f64 / (next_sunrise - sunset) as f64 * 6. + 6.
    } as usize;

    let hour_names: Vec<&str> = match opts.mode.as_ref() {
        "kanji" => KANJI,
        "zodiac" => ZODIAC,
        _ => LATIN,
    }
    .split(" ")
    .collect();

    let hour = hour_names[hour];

    println!("{}", hour);
}
