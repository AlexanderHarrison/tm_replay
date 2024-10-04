use clap::Parser;
use slp_parser::Stream;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// Import your library module
use tm_replay::{construct_tm_replay_from_slp, ReplayCreationError};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input Slippi replay file
    #[arg(short, long, value_name = "SLP_FILE")]
    slp_file: PathBuf,

    /// Start frame for the recording
    #[arg(short = 'f', long)]
    start_frame: usize,

    /// Number of frames to record
    #[arg(short = 'd', long)]
    num_frames: usize,

    /// Output filepath for the savestate
    #[arg(
        short,
        long,
        value_name = "OUTPUT_FILE",
        default_value = "new_recording.gci"
    )]
    output_file: PathBuf,

    /// Name to give to the recording (max 31 ASCII characters)
    #[arg(short, long, default_value = "new_recording")]
    name: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        // Optionally, you can use more specific exit codes
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read the Slippi replay file
    let mut slp_file = File::open(&args.slp_file)?;
    let mut slp_data = Vec::new();
    slp_file.read_to_end(&mut slp_data)?;

    // Parse the Slippi game
    let file_bytes = slp_data.as_slice();
    let game = match slp_parser::parse_file(&mut Stream::new(file_bytes)) {
        Ok((game, _)) => game,
        Err(e) => return Err(format!("Error: failed to parse slp file: {}", e).into()),
    };

    // Use the construct_tm_replay_from_slp function from your library
    match construct_tm_replay_from_slp(&game, args.start_frame, args.num_frames, &args.name) {
        Ok(savestate) => {
            // Write the output savestate file
            std::fs::write(&args.output_file, &savestate)?;
            println!("Savestate file created at {}", args.output_file.display());
            Ok(())
        }
        Err(e) => match e {
            ReplayCreationError::RecordingOutOfBounds => {
                return Err("Error: The specified frame range is out of bounds.".into());
            }
            ReplayCreationError::DurationTooLong => {
                return Err(
                    "Error: The duration exceeds the maximum allowed length (3600 frames).".into(),
                );
            }
            ReplayCreationError::FilenameTooLong => {
                return Err(
                    "Error: The provided name is too long (max 31 ASCII characters).".into(),
                );
            }
            ReplayCreationError::FilenameNotASCII => {
                return Err("Error: The provided name contains non-ASCII characters.".into());
            }
            ReplayCreationError::SpecialActionState => {
                return Err(
                    "Error: The character is in a special action state, which is not supported."
                        .into(),
                );
            }
            ReplayCreationError::ZeldaOnCpu => {
                return Err("Error: Zelda as CPU is not supported due to a known bug.".into());
            }
        },
    }
}
