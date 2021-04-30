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

fn throw(dice: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..dice)
}

fn main() {
    let opts: Opts = Opts::parse();
    
    if let Some(roll) = opts.roll {
        match roll {
            4 => println!("{}", throw(4)),
            6 => println!("{}", throw(6)),
            8 => println!("{}", throw(8)),
            10 => println!("{}", throw(10)),
            12 => println!("{}", throw(12)),
            20 => println!("{}", throw(20)),
            _ => println!("error: try rolling a 4, 6, 8, 10, 12 or 20.")
        }
    }
    
    if let Some(custom) = opts.custom {
        println!("{}", throw(custom));
    }
}
