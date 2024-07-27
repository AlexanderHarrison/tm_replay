use tm_replay::*;
use arrayvec::ArrayVec;

fn main() {
    let mut filename_buf = [0u8; 32];
    let name = b"DEBUG";
    filename_buf[0..name.len()].copy_from_slice(name);

    let (gci, filename) = construct_tm_replay(
        &RecordingInfo {
            hmn: slp_parser::Character::Peach.neutral(),
            cpu: slp_parser::CharacterColour::Sheik(slp_parser::character_colours::ZeldaColour::Green),
            stage: slp_parser::Stage::FinalDestination,
            time: RecordingTime::today_approx(),
            filename: filename_buf,
            menu_settings: RecordingMenuSettings {
                //hmn_mode: HmnRecordingMode::Playback,
                //hmn_slot: RecordingSlot::Slot1,
                ..RecordingMenuSettings::default()
            },
        },
        &InitialState {
            start_frame: 0,
            hmn: CharacterState {
                position: [0.0, 0.0],
                airborne: false,
                direction: Direction::Left,
                state: slp_parser::ActionState::Standard(slp_parser::StandardActionState::Wait),
                state_frame: 0.0,
                percent: 19.0,
                stale_moves: [StaleableMoves::DSmash; 10].into(),
                anim_velocity: [0.0; 2],
                self_velocity: [0.0; 2],
                hit_velocity: [0.0; 2],
                ground_velocity: [0.0; 2],
            },
            cpu: CharacterState {
                position: [10.0, 0.0],
                airborne: false,
                direction: Direction::Right,
                state: slp_parser::ActionState::Standard(slp_parser::StandardActionState::Wait),
                state_frame: 0.0,
                percent: 100.0,
                stale_moves: ArrayVec::new(),
                anim_velocity: [0.0; 2],
                self_velocity: [0.0; 2],
                hit_velocity: [0.0; 2],
                ground_velocity: [0.0; 2],
            },
        },
        &InputRecordings {
            hmn: &[],
            cpu: &[],
            //hmn: &[
            //    &[RecInputs {
            //        stick_x: -127i8,
            //        ..RecInputs::NO_INPUT
            //    }; 60],
            //    &[RecInputs {
            //        stick_x: 127i8,
            //        ..RecInputs::NO_INPUT
            //    }; 60],
            //],
            //cpu: &[
            //    &[RecInputs {
            //        stick_x: -127i8,
            //        ..RecInputs::NO_INPUT
            //    }; 60],
            //    &[RecInputs {
            //        stick_x: 127i8,
            //        ..RecInputs::NO_INPUT
            //    }; 60],
            //],
        },
    );

    let mut path = std::path::PathBuf::from("/home/alex/.config/SlippiOnline/GC/USA/Card A/");
    path.push(&filename);
    std::fs::write(&path, &gci).unwrap();
    println!("wrote replay {}", path.display());
}
