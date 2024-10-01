use tm_replay::*;

fn main() {
    let mut filename_buf = [0u8; 31];
    let name = b"DEBUG";
    filename_buf[0..name.len()].copy_from_slice(name);

    let gci = construct_tm_replay(
        &RecordingState {
            time: RecordingTime::today_approx(),
            filename: filename_buf,
            menu_settings: Default::default(),
            start_frame: 0,
            stage: slp_parser::Stage::FinalDestination,
            hmn_state: CharacterState {
                character: slp_parser::Character::Peach.neutral(),
                position: [-10.0, 0.0, 0.0],
                airborne: true,
                direction: slp_parser::Direction::Left,
                state: slp_parser::ActionState::Standard(slp_parser::StandardActionState::Dash),
                state_frame: 0.0,
                percent: 0.0,
                stale_moves: &[],
                anim_velocity: [0.0, 0.0, 0.0],
                self_velocity: [0.0, 0.0, 0.0],
                hit_velocity: [0.0; 3],
                ground_velocity: [0.0, 0.0, 0.0],
                hitlag_frames_left: 0.0,
                prev_position: [0.0; 3],
                stick: [0.0; 2],
                cstick: [0.0; 2],
                prev_stick: [0.0; 2],
                trigger: 0.0,
                state_speed: 0.01,
                state_blend: 0.0,
                x_rotn_rot: [-3.14159 / 2.0, 3.14159, 0.0, 0.0],
                last_ground_idx: 0,
            },
            cpu_state: CharacterState {
                character: slp_parser::CharacterColour::Peach(slp_parser::character_colours::PeachColour::Blue),
                position: [-10.0, 0.0, 0.0],
                airborne: true,
                direction: slp_parser::Direction::Left,
                state: slp_parser::ActionState::Standard(slp_parser::StandardActionState::Dash),
                state_frame: 0.0,
                percent: 0.0,
                stale_moves: &[],
                anim_velocity: [0.0, 0.0, 0.0],
                self_velocity: [0.0, 0.0, 0.0],
                hit_velocity: [0.0; 3],
                ground_velocity: [0.0, 0.0, 0.0],
                hitlag_frames_left: 0.0,
                prev_position: [0.0; 3],
                stick: [0.0; 2],
                cstick: [0.0; 2],
                prev_stick: [0.0; 2],
                trigger: 0.0,
                state_speed: 0.01,
                state_blend: 0.0,
                x_rotn_rot: [-3.14159 / 2.0, 0.0, 0.0, 0.0],
                last_ground_idx: 0,
            },
        },
        &InputRecordings {
            hmn_slots: [None; 6],
            cpu_slots: [None; 6],
        },
    );

    let mut path = std::path::PathBuf::from("/home/alex/.config/SlippiOnline/GC/USA/Card A/");
    path.push("rwing_export.gci");
    std::fs::write(&path, &gci).unwrap();
    println!("wrote replay {}", path.display());
}
