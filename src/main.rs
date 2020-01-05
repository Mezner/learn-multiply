use structopt::StructOpt;
#[macro_use] extern crate text_io;
extern crate termcolor;
extern crate rand;

use rand::Rng;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(StructOpt)]
#[structopt(name = "learn-multiply", about = "Practice your multiplication!")]
struct Opts {
    /// Minimum number
    #[structopt(name = "min", default_value = "1")]
    min: i32,
    /// Maximum number
    #[structopt(name = "max", default_value = "12")]
    max: i32,
    /// Number of problems to solve
    #[structopt(name = "problems", default_value = "25")]
    problems: usize
}

fn main() {
    let opts = Opts::from_args();
    // Lazy error checking.
    assert!(opts.min <= opts.max);
    let mut rng = rand::thread_rng();
    for i in 0..opts.problems {
        println!("Problem {} of {}", i + 1, opts.problems);
        println!("");
        let a = rng.gen_range(opts.min, opts.max);
        let b = rng.gen_range(opts.min, opts.max);
        problem(a, b);
    }
}

fn problem(a: i32, b: i32) {
    println!("  {: >3}", a);
    println!("x {: >3}", b);
    println!("-----");
    let result: i32 = read!();
    let expected = a * b;
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if result == expected {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
        println!("CORRECT!");
    } else {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
        println!("Nice try! Correct answer: {}", result);
    }
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    println!("");
}