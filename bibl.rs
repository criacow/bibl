use std::io;
use std::env;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::fs::File;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "process" {
        let infile = File::open(&args[2])?;
        let mut outfile = File::create(&args[3])?;
        let reader = BufReader::new(infile);
        let mut outbfr = String::from("");

        let mut linecount = 0;
        for line in reader.lines() {
            let l_str = line?;
            let items: Vec<_> = l_str
                .split_inclusive(&['.', '!', '?'][..])
                .collect();
            for i in items {
                let output = i.trim();
                linecount += 1;
                outbfr.push_str(output);
                outbfr.push('\n');
            }
        }
        write!(outfile, "{linecount}\n");
        write!(outfile, "{outbfr}\n");
    } else if command == "reading" {
        let infile = File::open(&args[2])?;
        let context: u64 = args[3].parse().expect("Failed to parse string to integer");
        let reader = BufReader::new(infile);

        let mut linereader = reader
            .lines()
            .map(|line| line.expect("Failed to read line!"));

        let linecount: u64 = linereader.next().unwrap().parse().unwrap();

        let hasher = RandomState::new().build_hasher();
        let randln = (hasher.finish() % linecount) + 1;
        let mut startln = 1;
        if randln > context {
            startln = randln - context;
        }
        let mut endln = randln + context;
        if endln > linecount {
            endln = linecount;
        }
        print!("File has {linecount} lines. Chosen line is number {randln}. Viewing lines {startln} to {endln}:\n");

        for (linenum, line) in linereader.enumerate() {
            let sln = startln as usize;
            let eln = endln as usize;
            if (linenum >= sln) && (linenum <= eln) {
                print!("{linenum} -- {line}\n");
            }
        }
    } else {
        print!("Unknown command. Commands are:\n");
        print!("  bibl process in.txt out.txt\n");
        print!("  bibl reading in.txt context_number\n");
    }

    Ok(())
}
