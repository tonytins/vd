use clap::{AppSettings, Clap};
use rand::Rng;

#[derive(Clap)]
#[clap(author, about, version)]
#[clap(setting = AppSettings::ColorAuto)]
struct Opts {
    #[clap(short, long)]
    roll: Option<i32>,
    #[clap(short, long)]
    custom: Option<i32>,
}

/// Rolls the dice
fn throw(dice: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..dice)
}

fn main() {
    let opts: Opts = Opts::parse();
    let msg = "You rolled a:";

    // -[r/roll] option
    if let Some(roll) = opts.roll {
        match roll {
            4 => println!("{} {}", msg, throw(4)),
            6 => println!("{} {}", msg, throw(6)),
            8 => println!("{} {}", msg, throw(8)),
            10 => println!("{} {}", msg, throw(10)),
            12 => println!("{} {}", msg, throw(12)),
            20 => println!("{} {}", msg, throw(20)),
            _ => println!("error: try rolling a 4, 6, 8, 10, 12 or 20."),
        }
    }

    // -[c/custom] option
    if let Some(custom) = opts.custom {
        println!("{} {}", msg, throw(custom));
    }
}
