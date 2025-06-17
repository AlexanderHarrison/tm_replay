const USAGE: &'static str = "fbin <good file> <bad file> <cmd> <start> <end> [check cmd args ...]";

// binary search two files to find crucial difference

fn main() {
    let mut args = std::env::args();
    args.next();

    let goodp = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("{}", USAGE);
            return;
        }
    };

    let badp = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("{}", USAGE);
            return;
        }
    };

    let range_start = match args.next().and_then(|n| n.parse::<usize>().ok()) {
        Some(p) => p,
        None => {
            eprintln!("{}", USAGE);
            return;
        }
    };

    let range_end = match args.next().and_then(|n| n.parse::<usize>().ok()) {
        Some(p) => p,
        None => {
            eprintln!("{}", USAGE);
            return;
        }
    };

    let check_cmd = args.collect::<Vec<String>>();

    let goodb = match std::fs::read(&goodp) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading {}: {}", &goodp, e);
            return;
        }
    };
    let badb = match std::fs::read(&badp) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading {}: {}", &badp, e);
            return;
        }
    };

    assert!(range_end >= range_start);

    let good_file_len = goodb.len();
    let bad_file_len = badb.len();
    assert!(good_file_len >= range_end);
    assert!(bad_file_len >= range_end);

    // start binsearch

    let mut scratch = badb.clone();
    
    // cannot binary search on range unfortunately
    let slice = (range_start..range_end).step_by(4).collect::<Vec<usize>>();

    fn run_cmd(cmd: &[String]) {
        if !cmd.is_empty() {
            std::process::Command::new(&cmd[0])
                .args(cmd[1..].iter())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    }

    let ask_pass_or_fail = || {
        loop {
            use std::io::Write;
            print!("Pass (p)/Fail (f)/Cancel (c): ");
            std::io::stdout().flush().unwrap();
            let mut buf = String::new();
            let io = std::io::stdin();
            io.read_line(&mut buf).unwrap();

            match buf.chars().next() {
                Some('p') => {
                    break true;
                },
                Some('f') => {
                    break false;
                },
                Some('c') => {
                    std::fs::write(&badp, &badb).unwrap();
                    std::process::exit(0);
                },
                _ => (),
            }
        }
    };

    let start_i = slice.partition_point(|i| {
        scratch.copy_from_slice(&badb);
        scratch[*i..range_end].copy_from_slice(&goodb[*i..range_end]);
        println!("range: 0x{:x}..0x{:x}", i, range_end);
        std::fs::write(&badp, &scratch).unwrap();

        run_cmd(&check_cmd);
        ask_pass_or_fail()
    }) - 1; // last idx that passes
    let start = slice[start_i];

    let end_i = slice[start_i..].partition_point(|i| {
        scratch.copy_from_slice(&badb);
        scratch[start..*i].copy_from_slice(&goodb[start..*i]);
        println!("range: 0x{:x}..0x{:x}", start, i);
        std::fs::write(&badp, &scratch).unwrap();

        run_cmd(&check_cmd);
        !ask_pass_or_fail()
    });
    let end = slice[start_i+end_i];

    println!("problem range: 0x{:x}..0x{:x}", start, end);

    std::fs::write(&badp, &badb).unwrap();
}
