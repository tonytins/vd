use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, help = "Roll from 4, 6, 8, 10, 12, 20 or a custom amount")]
    roll: Option<i32>,
}

/// Rolls the dice
fn throw(dice: i32) -> i32 {
    let mut rng = rand::thread_rng();
    if dice > i32::MAX {
        panic!("Dice amount too high")
    } else { // Should
        rng.gen_range(1..dice)
    }
}

fn main() {
    let opts: Args = Args::parse();
    let msg = "You rolled a";

    // -[r/roll] option
    if let Some(roll) = opts.roll {
        match roll {
            4 => println!("{} {}", msg, throw(4)),
            6 => println!("{} {}", msg, throw(6)),
            8 => println!("{} {}", msg, throw(8)),
            10 => println!("{} {}", msg, throw(10)),
            12 => println!("{} {}", msg, throw(12)),
            20 => println!("{} {}", msg, throw(20)),
            _ => println!("{} {}", msg, throw(roll)),
        }
    }
}
