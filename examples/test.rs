use tm_replay::*;

fn main() {
    let mut filename_buf = [0u8; 31];
    let name = b"rwing-export";
    filename_buf[0..name.len()].copy_from_slice(name);

    let gci = construct_tm_replay(
        &RecordingState {
            time: RecordingTime::today_approx(),
            filename: filename_buf,
            menu_settings: Default::default(),
            start_frame: 0,
            stage: slp_parser::Stage::FinalDestination,
            hmn_state: CharacterState {
                character: slp_parser::CharacterColour::Peach(slp_parser::character_colours::PeachColour::White),
                position: [0.0, 0.0, 0.0],
                ..Default::default()
            },
            cpu_state: CharacterState {
                character: slp_parser::Character::Zelda.neutral(),
                position: [10.0, 0.0, 0.0],
                ..Default::default()
            },
        },
        &InputRecordings {
            hmn_slots: [None; 6],
            cpu_slots: [None; 6],
        },
    ).unwrap();

    let mut path = std::path::PathBuf::from("./");
    path.push("rwing-export.gci");
    std::fs::write(&path, &gci).unwrap();
    println!("wrote replay {}", path.display());
}
