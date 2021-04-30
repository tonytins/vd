use clap::{AppSettings, Clap};
use rand::Rng;

#[derive(Clap)]
#[clap(author, about, version)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    roll: Option<i32>,
    #[clap(short, long)]
    custom: Option<i32>,
}

fn roll(dice: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..dice)
}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.roll.is_some() {
        match opts.roll.unwrap() {
            4 => println!("{}", roll(4)),
            6 => println!("{}", roll(6)),
            8 => println!("{}", roll(8)),
            10 => println!("{}", roll(10)),
            12 => println!("{}", roll(12)),
            _ => println!("{}", roll(20)),
        }
    } else if opts.custom.is_some() {
        println!("{}", roll(opts.custom.unwrap()));
    } else {
        println!("{}", roll(20));
    }
}
