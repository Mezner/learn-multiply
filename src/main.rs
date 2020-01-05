use structopt::StructOpt;
#[macro_use] extern crate text_io;
extern crate termcolor;
extern crate rand;

use rand::Rng;
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
    let problems = generate_problems(&opts);
    for (i, p) in problems.iter().enumerate() {
        println!("Problem {} of {}", i + 1, opts.problems);
        println!("");
        problem(p.0, p.1);
    }
}

fn generate_problems(opts: &Opts) -> Vec<(i32, i32)> {
    let mut problems = Vec::new();
    let mut rng = rand::thread_rng();
    while problems.len() < opts.problems {
        let a = rng.gen_range(opts.min, opts.max);
        let b = rng.gen_range(opts.min, opts.max);
        if !problems.contains(&(a, b)) {
            problems.push((a, b));
        }
    }
    problems
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