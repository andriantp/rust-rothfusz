use std::env;
use std::process;

mod rothfusz;
use rothfusz::{Repo, RepositoryI};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: cargo run -- <temp> <rh>");
        return;
    }

    let temp: f64 =
        match args[1].parse() {
            Ok(v) => v,

            Err(err) => {
                eprintln!(
                    "Failed to parse temp [{}]: {}",
                    args[1],
                    err,
                );

                process::exit(1);
            }
        };

    let rh: f64 =
        match args[2].parse() {
            Ok(v) => v,

            Err(err) => {
                eprintln!(
                    "Failed to parse rh [{}]: {}",
                    args[2],
                    err,
                );

                process::exit(1);
            }
        };

    let repo = Repo::new(
        26.7, // min valid temp
        85.0, // humid RH threshold
    );

    let result =
        repo.calculate_heat_index(temp, rh);

    println!("{:#?}", result);
}