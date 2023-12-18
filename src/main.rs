use std::io::{stdout, Write};

mod modules;
use modules::{
    args::Args,
    solver::{solve, SolverInfo},
};

const DICTIONARY: &str = include_str!("vivi.txt");

pub fn write_file(sinfo: SolverInfo, file: std::path::PathBuf) {
    let mut outfile = match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file)
    {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to access file: {}", e)
        }
    };

    match outfile.write_all(sinfo.get_header().as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            panic!("Failed to write to file: {}", e)
        }
    };

    sinfo.matches.iter().for_each(|m| {
        let text = format!("{}\n", m);

        match outfile.write_all(text.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to write to file: {}", e)
            }
        }
    });
}

fn main() {
    let args = Args::get();

    let dictionary = match args.dictionary {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(d) => d,
            Err(e) => {
                panic!("Failed to read dictionary file: {}", e)
            }
        },
        None => DICTIONARY.to_string(),
    };

    let solver_info = solve(args.prompt, dictionary, args.sorting);

    match args.file {
        Some(path) => write_file(solver_info, path),
        None => {
            let stdout = stdout();
            let mut handle = stdout.lock();

            match handle.write_all(solver_info.get_header().as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Failed to write to stdout: {}", e)
                }
            };

            solver_info.matches.iter().for_each(|m| {
                let text = format!("{}\n", m);

                match handle.write_all(text.as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        panic!("Failed to write to stdout: {}", e)
                    }
                }
            });
        }
    }
}
