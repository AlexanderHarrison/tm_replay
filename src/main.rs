use tm_replay::*;

const USAGE: &'static str = "Usage: tm_replay [OPTIONS] --slp-file <SLP_FILE> --start-frame <START_FRAME> --num-frames <NUM_FRAMES>

Options:
  -s, --slp-file <SLP_FILE>        Path to the input Slippi replay file
  -f, --start-frame <START_FRAME>  Start frame for the recording
  -n, --num-frames <NUM_FRAMES>    Number of frames to record [default: 360]
  -o, --output-file <OUTPUT_FILE>  Output filepath for the savestate [default: new_recording.gci]
  -n, --name <NAME>                Name to give to the recording (max 31 ASCII characters) [default: new_recording]
  -h, --help                       Print help
";

fn parse_str(args: &[String], i: &mut usize) -> Result<String, String> {
    match args.get(*i+1) {
        Some(s) => {
            *i += 2;
            Ok(s.clone())
        }
        None => Err(format!("Error: flag '{}' requires an argument", args[*i]).into())
    }
}

fn parse_num(args: &[String], i: &mut usize) -> Result<usize, String> {
    match args.get(*i+1) {
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n) => {
                    *i += 2;
                    Ok(n)
                }
                Err(_) => {
                    Err(format!("Error: '{}' is not a number", args[*i+1]).into())
                }
            }
        }
        None => Err(format!("Error: flag '{}' requires an argument", args[*i]).into())
    }
}

fn main() {
    match run() {
        Ok(_) => {},
        Err(e) => {
            eprint!("{}\n\n{}", e, USAGE);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() == 1 {
        print!("{}", USAGE);
        return Ok(());
    }

    let mut file = None;
    let mut start_frame = None;
    let mut num_frames = 360;
    let mut output_file = String::from("new_recording.gci");
    let mut name = String::from("new_recording");

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-s" | "--slp-file" => file = Some(parse_str(&args, &mut i)?),
            "-f" | "--start-frame" => start_frame = Some(parse_num(&args, &mut i)?),
            "-n" | "--num-frames" => num_frames = parse_num(&args, &mut i)?,
            "-o" | "--output-file" => output_file = parse_str(&args, &mut i)?,
            "-m" | "--name" => name = parse_str(&args, &mut i)?,
            "-h" | "--help" => {
                print!("{}", USAGE);
                return Ok(());
            }
            err => return Err(format!("Error: Unknown argument '{}'", err).into())
        }
    }

    let file = match file {
        Some(s) => s,
        None => return Err("Error: '--slp-file' argument is required".into()),
    };

    let start_frame = match start_frame {
        Some(s) => s,
        None => return Err("Error: '--start-frame' argument is required".into()),
    };

    let game = match slp_parser::read_game(std::path::Path::new(&file)) {
        Ok((game, _)) => game,
        Err(e) => return Err(format!("Error: failed to parse slp file: {}", e).into()),
    };

    match construct_tm_replay_from_slp(&game, HumanPort::HumanLowPort, start_frame, num_frames, &name) {
        Ok(savestate) => {
            std::fs::write(&output_file, &savestate)
                .map_err(|e| format!("Could not write output file '{}': {}", &output_file, e))?;
            println!("Savestate file '{}' created", &output_file);
            Ok(())
        }
        Err(e) => match e {
            ReplayCreationError::NotTwoPlayerGame => {
                return Err("Exports are only allowed in 1v1 replays.".into());
            }
            ReplayCreationError::RecordingOutOfBounds => {
                return Err("The specified frame range is out of bounds".into());
            }
            ReplayCreationError::DurationTooLong => {
                return Err(
                    "Error: The duration exceeds the maximum allowed length (3600 frames)".into(),
                );
            }
            ReplayCreationError::NoGoodExportFrame => {
                return Err(
                    "Error: Could not find a good export frame.".into(),
                );
            }
            ReplayCreationError::FilenameTooLong => {
                return Err(
                    "Error: The provided name is too long (max 31 ASCII characters)".into(),
                );
            }
            ReplayCreationError::FilenameNotASCII => {
                return Err("Error: The provided name contains non-ASCII characters".into());
            }
            ReplayCreationError::SpecialActionState => {
                return Err(
                    "Error: The character is in a special action state, which is not supported"
                        .into(),
                );
            }
            ReplayCreationError::ZeldaOnCpu => {
                return Err("Error: Zelda as CPU is not supported due to a known bug".into());
            }
            ReplayCreationError::OutdatedReplay => {
                return Err(format!(
                    "Error: Replay is out of date. Minimum slp version is {}.{}.0",
                    MIN_VERSION_MAJOR, MIN_VERSION_MINOR
                ));
            }
        },
    }
}
