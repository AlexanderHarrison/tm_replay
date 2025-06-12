// pub mod gen;
mod compress;
mod autocancel;
mod hitboxes;

pub const MIN_VERSION_MAJOR: u8 = 3;
pub const MIN_VERSION_MINOR: u8 = 16;

#[derive(Copy, Clone, Debug)]
pub enum RecordingSlot {
    Random = 0,
    Slot1 = 1, Slot2 = 2, Slot3 = 3, Slot4 = 4, Slot5 = 5, Slot6 = 6,
}

#[derive(Copy, Clone, Debug)]
pub enum HmnRecordingMode {
    Off = 0, Record = 1, Playback = 2
}

#[derive(Copy, Clone, Debug)]
pub enum CpuRecordingMode {
    Off = 0, Control = 1, Record = 2, Playback = 3
}

#[derive(Copy, Clone, Debug)]
pub struct RecordingMenuSettings {
    pub hmn_mode: HmnRecordingMode,
    pub hmn_slot: RecordingSlot,
    pub cpu_mode: CpuRecordingMode,
    pub cpu_slot: RecordingSlot,
    pub loop_inputs: bool,
    pub auto_restore: bool,
}

fn jump_count(c: slp_parser::Character) -> u8 {
    match c {
        slp_parser::Character::Jigglypuff | slp_parser::Character::Kirby => 6,
        _ => 2,
    }
}

impl Default for RecordingMenuSettings {
    fn default() -> Self {
        RecordingMenuSettings {
            hmn_mode: HmnRecordingMode::Off,
            hmn_slot: RecordingSlot::Slot1,
            cpu_mode: CpuRecordingMode::Off,
            cpu_slot: RecordingSlot::Slot1,
            loop_inputs: false,
            auto_restore: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RecordingTime {
    pub month : u8 ,
    pub day   : u8 ,
    pub year  : u16,
    pub hour  : u8 ,
    pub minute: u8 ,
    pub second: u8 ,
}

impl RecordingTime {
    pub fn today_approx() -> RecordingTime {
        use std::time::SystemTime;

        let seconds = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut days = seconds / (60 * 60 * 24);

        let mut year = 1970;
        let mut days_in_year;
        loop {
            days_in_year = 
                if year % 400 == 0 { 366 }
                else if year % 100 == 0 { 365 }
                else if year % 4 == 0 { 366 }
                else { 365 };

            if days_in_year <= days {
                days -= days_in_year;
                year += 1;
            } else {
                break;
            }
        }

        let month = loop {
            if days >= 31 { days -= 31 } else { break 1 }
            if days_in_year == 365 {
                if days >= 29 { days -= 29 } else { break 2 }
            }
            if days_in_year == 366 {
                if days >= 30 { days -= 30 } else { break 2 }
            }
            if days >= 31 { days -= 31 } else { break 3 }
            if days >= 30 { days -= 30 } else { break 4 }
            if days >= 31 { days -= 31 } else { break 5 }
            if days >= 30 { days -= 30 } else { break 6 }
            if days >= 31 { days -= 31 } else { break 7 }
            if days >= 31 { days -= 31 } else { break 8 }
            if days >= 30 { days -= 30 } else { break 9 }
            if days >= 31 { days -= 31 } else { break 10 }
            if days >= 30 { days -= 30 } else { break 11 }
            break 12
        };

        days += 1; // one-index the day

        RecordingTime {
            year,
            month,
            day: days as u8,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}

fn vector_to_arr(v: slp_parser::Vector) -> [f32; 2] { [v.x, v.y] }

/// this is the format dolphin uses for GCI filenames.
/// It's not necessary to name the recordings like this - any name will work.
pub fn dolphin_gci_filename(time: RecordingTime) -> String {
    format!(
        "01-GTME-TMREC_{:02}{:02}{:04}_{:02}{:02}{:02}.gci", 
        time.month, time.day, time.year,
        time.hour, time.minute, time.second,
    )
}

#[derive(Clone, Debug)]
pub struct RecordingState {
    pub time: RecordingTime,
    /// Name to show when browsing in Training Mode. Must be ascii.
    /// Maybe avoid special characters too, just in case.
    pub filename: [u8; 31],
    pub menu_settings: RecordingMenuSettings,

    /// Melee starts at frame -123. 'GO' disappears on frame 0.
    pub start_frame: i32,
    pub stage: slp_parser::Stage,
    pub hmn_state: CharacterState,
    /// The stale moves and costume fields will be ignored.
    pub hmn_follower_state: Option<CharacterState>,
    pub cpu_state: CharacterState,
    /// The stale moves and costume fields will be ignored.
    pub cpu_follower_state: Option<CharacterState>,
}

impl RecordingState {
    // offsets zeroed but not written
    fn write_header(&self, b: &mut Vec<u8>, swap_shiek_zelda: bool) {
        let char_hmn = if swap_shiek_zelda {
            // We swap zelda and shiek to work around bugs in Unclepunch prior to TM-CE v1.3.
            //
            // The default savestate match init was generated with two sheiks on FD.
            // Other character combinations tend to crash with zelda and sheik
            // or have the unused transformation tpose in the centre
            // This combination doesn't seem to have these issues, but it prevents using zelda on cpu.
            match self.hmn_state.character.character() {
                slp_parser::Character::Zelda => slp_parser::Character::Sheik,
                slp_parser::Character::Sheik => slp_parser::Character::Zelda,
                c => c,
            }.to_u8_external().unwrap()
        } else {
            self.hmn_state.character.character().to_u8_external().unwrap()
        };

        let costume_hmn = self.hmn_state.character.costume_idx();
        let char_cpu = self.cpu_state.character.character().to_u8_external().unwrap();
        let costume_cpu = self.cpu_state.character.costume_idx();

        let stage_external = self.stage.to_u16_external().to_be_bytes();
        let stage_internal = self.stage.to_u16_internal().to_be_bytes();

        let year = self.time.year.to_be_bytes();

        b.extend_from_slice(&[
            0, 1, // version
            0, 96,// image width
            0, 72,// image height
            0,  4,// image fmt
            char_hmn,
            costume_hmn,
            char_cpu,
            costume_cpu,
            stage_external[0], stage_external[1],
            stage_internal[0], stage_internal[1],
            self.time.month,
            self.time.day,
            year[0], year[1],
            self.time.hour,
            self.time.minute,
            self.time.second,
        ]);

        b.extend_from_slice(&self.filename);
        b.extend_from_slice(&[0u8; 1]); // 1 byte padding
        b.extend_from_slice(&[0u8; 12]); // 3 offsets
    }
    
    fn write_menu_settings(&self, b: &mut Vec<u8>) {
        b.extend_from_slice(&[
            self.menu_settings.hmn_mode as u8,
            self.menu_settings.hmn_slot as u8,
            self.menu_settings.cpu_mode as u8,
            self.menu_settings.cpu_slot as u8,
            self.menu_settings.loop_inputs as u8,
            self.menu_settings.auto_restore as u8,
        ]);
    }
}

#[derive(Clone, Debug)]
/// Initial state for a character.
///
/// Note that this struct has a Default implementation.
pub struct CharacterState {
    pub character: slp_parser::CharacterColour,
    pub position: [f32; 3],
    pub prev_position: [f32; 3],
    pub airborne: bool,
    pub state: slp_parser::ActionState,
    pub state_frame: f32,
    pub state_speed: f32,
    /// Between 0 and 1, where 0 is no blending.
    pub state_blend: f32,
    /// I could not tell you how to use this.
    /// Controls character rotation. Keep zeroed to be safe.
    /// Nonzero in very specific circumstances.
    /// Incomplete list of actions that are nonzero:
    /// - Peach's nair
    pub x_rotn_rot: [f32; 4],
    pub direction: slp_parser::Direction,
    pub percent: f32,
    pub last_ground_idx: u32,
    pub jumps_remaining: u8,
    /// Zero if n/a. See https://docs.google.com/spreadsheets/d/1spibzWaitiA22s7db1AEw1hqQXzPDNFZHYjc4czv2dc
    pub stale_moves: [slp_parser::StaleMove; 10],
    pub anim_velocity: [f32; 3],
    pub self_velocity: [f32; 3],
    pub hit_velocity: [f32; 3],
    pub ground_velocity: [f32; 3],

    /// number of frames in knockback if in knockback, otherwise -1
    pub frames_since_hit: i32,

    /// number of consecutive frames offscreen. Counts to 60 then the player takes damage.
    pub offscreen_damage_timer: u32,

    /// Generic character state variables, used for special actions.
    ///
    /// ## 4..8
    /// - frames of float left
    pub char_fighter_var: [u8; 208],

    /// Generic character state variables, used for most actions.
    ///
    /// ## 0..4
    /// - During hitstun and hitstop: the number of frames of hitstun remaining
    /// - During turn: set to 1 when actionable (???)
    ///
    /// ## 4..8
    /// - During turn: set to -1.0 if turning left or 1.0 if turning right
    pub char_state_var: [u8; 72],

    /// State flags. 
    pub subaction_flags: [u8; 16],

    /// State flags. 
    /// See https://github.com/project-slippi/slippi-wiki/blob/master/SPEC.md#state-bit-flags-1 for more information.
    pub state_flags: [u8; 5],

    pub hitlag_frames_left: f32,
    pub stick: [f32; 2],
    pub cstick: [f32; 2],
    pub prev_stick: [f32; 2],
    pub held: u16,
    pub prev_held: u16,
    pub trigger: f32,
    pub last_lstick_x_direction: slp_parser::Direction,
    pub input_timers: InputTimers,
}

#[derive(Copy, Clone, Debug)]
#[allow(non_snake_case)]
pub struct InputTimers {
    pub timer_lstick_tilt_x             : u8,
    pub timer_lstick_tilt_y             : u8,
    pub timer_trigger_analog            : u8,
    pub timer_lstick_smash_x            : u8,
    pub timer_lstick_smash_y            : u8,
    pub timer_trigger_digital           : u8,
    pub timer_lstick_any_x              : u8,
    pub timer_lstick_any_y              : u8,
    pub timer_trigger_any               : u8,
    pub x679_x                          : u8,
    pub x67A_y                          : u8,
    pub x67B                            : u8,
    pub timer_a                         : u8,
    pub timer_b                         : u8,
    pub timer_xy                        : u8,
    pub timer_trigger_any_ignore_hitlag : u8,
    pub timer_LR                        : u8,
    pub timer_padup                     : u8,
    pub timer_paddown                   : u8,
    pub timer_item_release              : u8,
    pub since_rapid_lr                  : u8,
    pub timer_jump                      : u8,
    pub timer_specialhi                 : u8,
    pub timer_speciallw                 : u8,
    pub timer_specials                  : u8,
    pub timer_specialn                  : u8,
    pub timer_jump_lockout              : u8,
    pub timer_specialhi_lockout         : u8,
}

impl InputTimers {
    // taken from decomp src/melee/ft/fighter.c
    pub fn advance(&mut self, frame: &slp_parser::Frame, frame_prev: &slp_parser::Frame) {
        self.timer_lstick_any_x += 1;
        if self.timer_lstick_any_x > 0xFE {
            self.timer_lstick_any_x = 0xFE;
        }

        let lstick = frame.left_stick_coords;
        let lstick1 = frame_prev.left_stick_coords;

        if lstick.x >= 0.25 {
            if lstick1.x >= 0.25 {
                self.timer_lstick_tilt_x += 1;
                if self.timer_lstick_tilt_x > 0xFE {
                    self.timer_lstick_tilt_x = 0xFE;
                }
                self.timer_lstick_smash_x += 1;
                if self.timer_lstick_smash_x > 0xFE {
                    self.timer_lstick_smash_x = 0xFE;
                }
                self.x679_x += 1;
                if self.x679_x > 0xFE {
                    self.x679_x = 0xFE;
                }
            } else {
                self.timer_lstick_any_x = 0;
                self.timer_lstick_smash_x = 0;
                self.timer_lstick_tilt_x = 0;
            }
        } else if lstick.x <= -0.25 {
            if lstick1.x <= -0.25 {
                self.timer_lstick_tilt_x += 1;
                if self.timer_lstick_tilt_x > 0xFE {
                    self.timer_lstick_tilt_x = 0xFE;
                }
                self.timer_lstick_smash_x += 1;
                if self.timer_lstick_smash_x > 0xFE {
                    self.timer_lstick_smash_x = 0xFE;
                }
                self.x679_x += 1;
                if self.x679_x > 0xFE {
                    self.x679_x = 0xFE;
                }
            } else {
                self.timer_lstick_any_x = 0;
                self.timer_lstick_smash_x = 0;
                self.timer_lstick_tilt_x = 0;
            }
        } else {
            self.x679_x = 0xFE;
            self.timer_lstick_smash_x = 0xFE;
            self.timer_lstick_tilt_x = 0xFE;
        }

        self.timer_lstick_any_y += 1;
        if self.timer_lstick_any_y > 0xFE {
            self.timer_lstick_any_y = 0xFE;
        }

        if lstick.y >= 0.25 {
            if lstick1.y >= 0.25 {
                self.timer_lstick_tilt_y += 1;
                if self.timer_lstick_tilt_y > 0xFE {
                    self.timer_lstick_tilt_y = 0xFE;
                }
                self.timer_lstick_smash_y += 1;
                if self.timer_lstick_smash_y > 0xFE {
                    self.timer_lstick_smash_y = 0xFE;
                }
                self.x67A_y += 1;
                if self.x67A_y > 0xFE {
                    self.x67A_y = 0xFE;
                }
            } else {
                self.timer_lstick_any_y = 0;
                self.timer_lstick_smash_y = 0;
                self.timer_lstick_tilt_y = 0;
            }
        } else if lstick.y <= -0.25 {
            if lstick1.y <= -0.25 {
                self.timer_lstick_tilt_y += 1;
                if self.timer_lstick_tilt_y > 0xFE {
                    self.timer_lstick_tilt_y = 0xFE;
                }
                self.timer_lstick_smash_y += 1;
                if self.timer_lstick_smash_y > 0xFE {
                    self.timer_lstick_smash_y = 0xFE;
                }
                self.x67A_y += 1;
                if self.x67A_y > 0xFE {
                    self.x67A_y = 0xFE;
                }
            } else {
                self.timer_lstick_any_y = 0;
                self.timer_lstick_smash_y = 0;
                self.timer_lstick_tilt_y = 0;
            }
        } else {
            self.x67A_y = 0xFE;
            self.timer_lstick_smash_y = 0xFE;
            self.timer_lstick_tilt_y = 0xFE;
        }

        if lb_8000D148(lstick1.x, lstick1.y, lstick.x, lstick.y, 0.0, 0.0, 0.25) {
            self.x67A_y = 0;
            self.x679_x = 0;
        }

        self.timer_trigger_any += 1;
        if self.timer_trigger_any > 0xFE {
            self.timer_trigger_any = 0xFE;
        }

        if frame.analog_trigger_value >= 0.25 {
            if frame.analog_trigger_value >= 0.25 {
                self.timer_trigger_analog += 1;
                if self.timer_trigger_analog > 0xFE {
                    self.timer_trigger_analog = 0xFE;
                }
                self.timer_trigger_digital += 1;
                if self.timer_trigger_digital > 0xFE {
                    self.timer_trigger_digital = 0xFE;
                }
                self.x67B += 1;
                if self.x67B > 0xFE {
                    self.x67B = 0xFE;
                }
            } else {
                self.x67B = 0;
                self.timer_trigger_any = 0;
                self.timer_trigger_digital = 0;
                self.timer_trigger_analog = 0;
            }
        } else {
            self.x67B = 0xFE;
            self.timer_trigger_digital = 0xFE;
            self.timer_trigger_analog = 0xFE;
        }

        let down = frame.buttons_mask;
        use slp_parser::buttons_mask as button;

        if down & button::A != 0 {
            self.timer_item_release = self.timer_a;
            self.timer_a = 0;
        } else if self.timer_a < 0xFF {
            self.timer_a += 1;
        }

        if down & button::B != 0 {
            self.timer_b = 0;
        } else if self.timer_b < 0xFF {
            self.timer_b += 1;
        }

        if down & (button::X | button::Y) != 0 {
            self.timer_xy = 0;
        } else if self.timer_xy < 0xFF {
            self.timer_xy += 1;
        }

        if down & button::D_PAD_UP != 0 {
            self.timer_padup = 0;
        } else if self.timer_padup < 0xFF {
            self.timer_padup += 1;
        }

        if down & button::D_PAD_DOWN != 0 {
            self.timer_paddown = 0;
        } else if self.timer_paddown < 0xFF {
            self.timer_paddown += 1;
        }

        // THIS BUTTON DOESNT EXIST IN MEX OR SLP????
        //if (down & 0x80000000) {
        //    self.timer_trigger_any_ignore_hitlag = 0;
        //} else if (self.timer_trigger_any_ignore_hitlag < 0xFF) {
        //    self.timer_trigger_any_ignore_hitlag += 1;
        //}

        if down & (button::L_DIGITAL | button::R_DIGITAL) != 0 {
            self.since_rapid_lr = self.timer_LR;
            self.timer_LR = 0;
        } else if self.timer_LR < 0xFF {
            self.timer_LR += 1;
        }
    }
}

// taken from decomp src/melee/lb/lb_00CE.c
// IDK what this does.
#[allow(non_snake_case)]
fn lb_8000D148(
    point0_x: f32, point0_y: f32,
    point1_x: f32, point1_y: f32,
    point2_x: f32, point2_y: f32,
    threshold: f32
) -> bool {
    let dist_01;
    let mut var_f0;
    {
        let diff_01_y = point0_y - point1_y;
        let diff_01_x = point1_x - point0_x;
        let dist_squared_01 = diff_01_x * diff_01_x + diff_01_y * diff_01_y;
        if dist_squared_01 < 0.00001 {
            return false;
        }
        dist_01 = dist_squared_01.sqrt();

        var_f0 = (point0_x * point1_y - point0_y * point1_x)
                 + (diff_01_x * point2_x + diff_01_y * point2_y);
        if var_f0 < 0.0 {
            var_f0 = -var_f0;
        }
    }

    if (var_f0 / dist_01) <= threshold {
        let diff_02_x = point0_x - point2_x;
        let diff_02_y = point0_y - point2_y;
        let diff_12_x = point1_x - point2_x;
        let diff_12_y = point1_y - point2_y;
        let threshold_squared = threshold * threshold;
        let dist_squared_02 = diff_02_x * diff_02_x + diff_02_y * diff_02_y;
        let dist_squared_12 = diff_12_x * diff_12_x + diff_12_y * diff_12_y;
        if dist_squared_02 < threshold_squared {
            if dist_squared_12 > threshold_squared {
                return true;
            }
            if dist_squared_12 < threshold_squared {
                return false;
            }
            return true;
        }
        if dist_squared_02 > threshold_squared {
            if dist_squared_12 > threshold_squared {
                // If an axis of point0 and point1 are on opposite sides of
                // point2, return true.
                if  ((point0_x > point2_x) && (point1_x < point2_x)) ||
                    ((point0_x < point2_x) && (point1_x > point2_x)) ||
                    ((point0_y > point2_y) && (point1_y < point2_y)) ||
                    ((point0_y < point2_y) && (point1_y > point2_y))
                {
                    return true;
                }
                return false;
            }
            if dist_squared_12 < threshold_squared {
                return true;
            }
            return true;
        }
        return true;
    }
    return false;
}

impl Default for InputTimers {
    fn default() -> Self {
        InputTimers {
            timer_lstick_tilt_x             : 0xFE,
            timer_lstick_tilt_y             : 0xFE,
            timer_trigger_analog            : 0xFE,
            timer_lstick_smash_x            : 0xFE,
            timer_lstick_smash_y            : 0xFE,
            timer_trigger_digital           : 0xFE,
            timer_lstick_any_x              : 0xFE,
            timer_lstick_any_y              : 0xFE,
            timer_trigger_any               : 0xFE,
            x679_x                          : 0xFE,
            x67A_y                          : 0xFE,
            x67B                            : 0xFE,
            timer_a                         : 0xFF,
            timer_b                         : 0xFF,
            timer_xy                        : 0xFF,
            timer_trigger_any_ignore_hitlag : 0xFF,
            timer_LR                        : 0xFF,
            timer_padup                     : 0xFF,
            timer_paddown                   : 0xFF,
            timer_item_release              : 0xFF,
            since_rapid_lr                  : 0xFF,
            timer_jump                      : 0xFF,
            timer_specialhi                 : 0xFF,
            timer_speciallw                 : 0xFF,
            timer_specials                  : 0xFF,
            timer_specialn                  : 0xFF,
            timer_jump_lockout              : 0xFF,
            timer_specialhi_lockout         : 0xFF,
        }
    }
}

impl Default for CharacterState {
    fn default() -> Self {
        CharacterState {
            character: slp_parser::Character::Peach.neutral(),
            position: [0.0, 0.0, 0.0],
            airborne: false,
            direction: slp_parser::Direction::Left,
            state: slp_parser::ActionState::Standard(slp_parser::StandardActionState::Wait),
            state_frame: 0.0,
            jumps_remaining: 0,
            percent: 0.0,
            stale_moves: [slp_parser::StaleMove::NULL; 10],
            anim_velocity: [0.0; 3],
            self_velocity: [0.0; 3],
            hit_velocity: [0.0; 3],
            ground_velocity: [0.0; 3],
            frames_since_hit: -1,
            offscreen_damage_timer: 0,
            hitlag_frames_left: 0.0,
            char_fighter_var: [0u8; 208],
            char_state_var: [0u8; 72],
            subaction_flags: [0u8; 16],
            prev_position: [0.0; 3],
            stick: [0.0; 2],
            cstick: [0.0; 2],
            prev_stick: [0.0; 2],
            input_timers: InputTimers::default(),
            held: 0,
            prev_held: 0,
            state_flags: [0; 5],
            trigger: 0.0,
            state_speed: 1.0,
            state_blend: 0.0,
            x_rotn_rot: [0.0, 0.0, 0.0, 0.0],
            last_lstick_x_direction: slp_parser::Direction::Left,
            last_ground_idx: 0,
        }
    }
}

pub mod buttons {
    pub const Z: u8 = 0x01;
    pub const R: u8 = 0x02;
    pub const L: u8 = 0x04;
    pub const X: u8 = 0x08;
    pub const Y: u8 = 0x10;
    pub const B: u8 = 0x20;
    pub const A: u8 = 0x40;
    pub const DPAD_UP: u8 = 0x80;
}

// the SLP file has buttons in a different layout.
// We need to translate it into UP's layout.
pub fn translate_buttons_from_slp(mask: u16) -> u8 {
    use slp_parser::buttons_mask;

    let translated = ((mask & buttons_mask::Z) >> 4)
      | ((mask & buttons_mask::R_DIGITAL) >> 4)
      | ((mask & buttons_mask::L_DIGITAL) >> 4)
      | ((mask & buttons_mask::X) >> 6)
      | ((mask & buttons_mask::Y) >> 8)
      | ((mask & buttons_mask::B) >> 4)
      | ((mask & buttons_mask::A) >> 2)
      | ((mask & buttons_mask::D_PAD_UP) << 4);
    translated as u8
}

#[derive(Copy, Clone, Debug)]
pub struct Input {
    /// - z: 0x01
    /// - r digital: 0x02
    /// - l digital: 0x04
    /// - x: 0x08
    /// - y: 0x10
    /// - b: 0x20
    /// - a: 0x40
    /// - dpad up: 0x80
    pub button_flags: u8,

    /// ensure value is within [-80, 80]
    pub stick_x: i8,
    /// ensure value is within [-80, 80]
    pub stick_y: i8,
    /// ensure value is within [-80, 80]
    pub cstick_x: i8,
    /// ensure value is within [-80, 80]
    pub cstick_y: i8,

    /// ensure value is within [0, 140]
    pub trigger: u8,
}

impl Input {
    pub const NONE: Input = Input {
        button_flags: 0, stick_x: 0, stick_y: 0, cstick_x: 0, cstick_y: 0, trigger: 0,
    };

    pub fn add(mut self, buttons: u8) -> Self {
        self.button_flags |= buttons;
        self
    }

    pub fn stick(mut self, stick_x: i8, stick_y: i8) -> Self {
        self.stick_x = stick_x;
        self.stick_y = stick_y;
        self
    }
}

#[derive(Copy, Clone, Debug)]
pub struct InputRecordings<'a> {
    /// Each slot is truncated to 3600 frames.
    pub hmn_slots: [Option<&'a [Input]>; 6],

    /// Each slot is truncated to 3600 frames.
    pub cpu_slots: [Option<&'a [Input]>; 6],
}

#[derive(Copy, Clone, Debug)]
pub enum ReplayCreationError {
    OutdatedReplay,
    NotTwoPlayerGame,
    RecordingOutOfBounds,
    DurationTooLong,
    FilenameTooLong,
    FilenameNotASCII,
    SpecialActionState,
    NoGoodExportFrame,
    ZeldaOnCpu,
}

const EVENT_DATASIZE: usize = 512;
const REC_LENGTH: usize = 1 * 60 * 60; // 60 seconds
const REC_SLOTS: usize = 6;
const REC_SLOT_SIZE: usize = 4 + 4 + REC_LENGTH*6;

static DEFAULT_SAVESTATE_AND_MATCHINIT: &'static [u8] = include_bytes!("savestate_matchinit.raw");
const SAVESTATE_SIZE: usize = 54616;
const MATCHINIT_SIZE: usize = 312;
const RECORDING_SIZE: usize = 314224;

static DEFAULT_GCI_HEADER: &'static [u8] = include_bytes!("gci_header.raw");

const BLOCK_SIZE: usize = 8192;

// These are computed from the Start.dol. See example `extract_fn_table`.
static ACTION_FN_LOOKUP_TABLE: &'static [u8] = include_bytes!("fn_table.raw");
static SPECIAL_ACTION_FN_LOOKUP_TABLE: &'static [u8] = include_bytes!("special_fn_table.raw");
static SPECIAL_ACTION_FN_CHARACTER_OFFSETS: [u16; 27] = [
    0x0000, 0x0140, 0x05a0, 0x0880, 0x0e20, 0x26c0, 0x29a0, 0x2c40, 0x2f40,
    0x33c0, 0x3780, 0x3aa0, 0x3dc0, 0x4100, 0x4340, 0x46c0, 0x4ac0, 0x4d40,
    0x4f80, 0x5380, 0x55c0, 0x5860, 0x59a0, 0x5e00, 0x6140, 0x6640, 0x6920,
];

fn calculate_checksum(src: &[u8], result: &mut [u8]) {
    let mut checksum: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10
    ];
        
    for i in 0..src.len() {
        checksum[i % 16] = checksum[i % 16].wrapping_add(src[i]);
    }
        
    for i in 1..16 {
        if checksum[i-1] == checksum[i] {
            checksum[i] ^= 0xFF;
        }
    }

    result[0..16].copy_from_slice(&checksum);
}

static ENCODE_LUT: [u32; 13] = [
    0x26, 0xFF, 0xE8, 0xEF, 0x42, 0xD6, 0x01, 0x54, 0x14, 0xA3, 0x80, 0xFD, 0x6E
];

fn obfuscate_byte(prev: u8, this: u8) -> u8 {
    let r3 = prev as u32;
    let mut r4 = this as u32;

    let b: u32 = r3 & 0xFF;
    r4 = b ^ r4;
    r4 ^= ENCODE_LUT[(b % 13) as usize];

    let r5;
    match b % 7 {
        0 => {
            r5 = ((r4 & 0x01) << 0) |
                 ((r4 & 0x02) << 3) |
                 ((r4 & 0x04) >> 1) |
                 ((r4 & 0x08) << 2) |
                 ((r4 & 0x10) >> 2) |
                 ((r4 & 0x20) << 1) |
                 ((r4 & 0x40) >> 3) |
                 ((r4 & 0x80) >> 0);
            r4 = r5 & 0xFF;
            return r4 as u8;
        }
        1 => {
            r5 = ((r4 & 0x01) << 3) |
                 ((r4 & 0x02) >> 1) |
                 ((r4 & 0x04) << 0) |
                 ((r4 & 0x08) << 3) |
                 ((r4 & 0x10) << 1) |
                 ((r4 & 0x20) >> 1) |
                 ((r4 & 0x40) << 1) |
                 ((r4 & 0x80) >> 6);
            r4 = r5 & 0xFF;
            return r4 as u8;
        }
        2 => {
            r5 = ((r4 & 0x01) << 6) |
                 ((r4 & 0x02) << 4) |
                 ((r4 & 0x04) >> 2) |
                 ((r4 & 0x08) >> 2) |
                 ((r4 & 0x10) >> 1) |
                 ((r4 & 0x20) << 2) |
                 ((r4 & 0x40) >> 4) |
                 ((r4 & 0x80) >> 3);
            r4 = r5 & 0xFF;
            return r4 as u8;
        }
        3 => {
            r5 = ((r4 & 0x01) << 1) |
                 ((r4 & 0x02) << 2) |
                 ((r4 & 0x04) << 5) |
                 ((r4 & 0x08) << 1) |
                 ((r4 & 0x10) >> 4) |
                 ((r4 & 0x20) >> 3) |
                 ((r4 & 0x40) >> 1) |
                 ((r4 & 0x80) >> 1);
            r4 = r5 & 0xFF;
            return r4 as u8;
        }
        4 => {
            r5 = ((r4 & 0x01) << 7) |
                 ((r4 & 0x02) << 1) |
                 ((r4 & 0x04) << 3) |
                 ((r4 & 0x08) >> 3) |
                 ((r4 & 0x10) << 2) |
                 ((r4 & 0x20) >> 4) |
                 ((r4 & 0x40) >> 2) |
                 ((r4 & 0x80) >> 4);
            r4 = r5 & 0xFF;
            return r4 as u8;
        }
        5 => {
            r5 = ((r4 & 0x01) << 5) |
                 ((r4 & 0x02) << 5) |
                 ((r4 & 0x04) << 2) |
                 ((r4 & 0x08) >> 0) |
                 ((r4 & 0x10) << 3) |
                 ((r4 & 0x20) >> 5) |
                 ((r4 & 0x40) >> 5) |
                 ((r4 & 0x80) >> 5);
            r4 = r5 & 0xFF;
            return r4 as u8;
        }
        6 => {
            r5 = ((r4 & 0x01) << 2) |
                 ((r4 & 0x02) << 0) |
                 ((r4 & 0x04) << 4) |
                 ((r4 & 0x08) << 4) |
                 ((r4 & 0x10) << 0) |
                 ((r4 & 0x20) >> 2) |
                 ((r4 & 0x40) >> 6) |
                 ((r4 & 0x80) >> 2);
            r4 = r5 & 0xFF;
            return r4 as u8;
        }
        _ => unreachable!()
    }
}

fn encode_block(data: &mut [u8]) {
    let len = data.len();
    let (checksum, encoded_data) = data.split_at_mut(16);
    calculate_checksum(encoded_data, checksum);

    for i in 16..len {
        data[i] = obfuscate_byte(data[i-1], data[i]);
    }
}

const WEIRD_BLOCK_HEADER: [u8; 16] = [
    0, 0, // block idx
    0, 0, 0xc0, // idk
    0, 0, // total size
    1, 0, 0, 0, 2, 0, 0, 0, 0, // idk
];

fn deobfuscate_byte(r3: u8, r4: u8) -> u8 {
    let b = r3 as u32;
    let mut r4 = r4 as u32;

    let r5;
    match b % 7 {
        0 => {
            r5 = ((r4 & 0x01) << 0) |
                 ((r4 & 0x02) << 1) |
                 ((r4 & 0x04) << 2) |
                 ((r4 & 0x08) << 3) |
                 ((r4 & 0x10) >> 3) |
                 ((r4 & 0x20) >> 2) |
                 ((r4 & 0x40) >> 1) |
                 ((r4 & 0x80) >> 0);
            r4 = r5 & 0xFF;
        }
        1 => {
            r5 = ((r4 & 0x01) << 1) |
                 ((r4 & 0x02) << 6) |
                 ((r4 & 0x04) << 0) |
                 ((r4 & 0x08) >> 3) |
                 ((r4 & 0x10) << 1) |
                 ((r4 & 0x20) >> 1) |
                 ((r4 & 0x40) >> 3) |
                 ((r4 & 0x80) >> 1);
            r4 = r5 & 0xFF;
        }
        2 => {
            r5 = ((r4 & 0x01) << 2) |
                 ((r4 & 0x02) << 2) |
                 ((r4 & 0x04) << 4) |
                 ((r4 & 0x08) << 1) |
                 ((r4 & 0x10) << 3) |
                 ((r4 & 0x20) >> 4) |
                 ((r4 & 0x40) >> 6) |
                 ((r4 & 0x80) >> 2);
            r4 = r5 & 0xFF;
        }
        3 => {
            r5 = ((r4 & 0x01) << 4) |
                 ((r4 & 0x02) >> 1) |
                 ((r4 & 0x04) << 3) |
                 ((r4 & 0x08) >> 2) |
                 ((r4 & 0x10) >> 1) |
                 ((r4 & 0x20) << 1) |
                 ((r4 & 0x40) << 1) |
                 ((r4 & 0x80) >> 5);
            r4 = r5 & 0xFF;
        }
        4 => {
            r5 = ((r4 & 0x01) << 3) |
                 ((r4 & 0x02) << 4) |
                 ((r4 & 0x04) >> 1) |
                 ((r4 & 0x08) << 4) |
                 ((r4 & 0x10) << 2) |
                 ((r4 & 0x20) >> 3) |
                 ((r4 & 0x40) >> 2) |
                 ((r4 & 0x80) >> 7);
            r4 = r5 & 0xFF;
        }
        5 => {
            r5 = ((r4 & 0x01) << 5) |
                 ((r4 & 0x02) << 5) |
                 ((r4 & 0x04) << 5) |
                 ((r4 & 0x08) >> 0) |
                 ((r4 & 0x10) >> 2) |
                 ((r4 & 0x20) >> 5) |
                 ((r4 & 0x40) >> 5) |
                 ((r4 & 0x80) >> 3);
            r4 = r5 & 0xFF;
        }
        6 => {
            r5 = ((r4 & 0x01) << 6) |
                 ((r4 & 0x02) << 0) |
                 ((r4 & 0x04) >> 2) |
                 ((r4 & 0x08) << 2) |
                 ((r4 & 0x10) << 0) |
                 ((r4 & 0x20) << 2) |
                 ((r4 & 0x40) >> 4) |
                 ((r4 & 0x80) >> 4);
            r4 = r5 & 0xFF;
        }
        _ => unreachable!(),
    }

    r4 ^= ENCODE_LUT[(b % 13) as usize];
    r4 ^= r3 as u32;
    return r4 as u8;
}


fn decode_block(src: &mut [u8]) -> i32 {
    let mut checksum = [0u8; 16];
    let mut x = src[15];
    for i in 16..src.len() {
        let y = src[i];
        src[i] = deobfuscate_byte(x, y);
        x = y;
    }
    calculate_checksum(&src[16..], &mut checksum);

    for i in 0..16 {
        if src[i] != checksum[i] {
            return -1;
        }
    }
        
    return 0;
}

// panic on invalid gci file.
pub fn read_replay_buffer(gci_file: &mut [u8]) -> Vec<u8> {
    let block_count = u16::from_be_bytes(gci_file[0x38..0x3A].try_into().unwrap()) as usize;
    let start = 0x1EB0;
    let decoded_len = 400 - 32 + (block_count-1)*(BLOCK_SIZE - 32);
    let mut decoded = Vec::with_capacity(decoded_len);

    assert!(decode_block(&mut gci_file[start..][..400]) == 0);

    // skip checksum and metadata
    decoded.extend_from_slice(&gci_file[start+32..][..400-32]);

    for i in 1..block_count {
        let block_start = start + 400 + (i-1)*BLOCK_SIZE;
        assert!(decode_block(&mut gci_file[block_start..][..BLOCK_SIZE]) == 0);
        decoded.extend_from_slice(&gci_file[block_start+32..][..BLOCK_SIZE-32]);
    }

    decoded
}

/// Overwrites the RecordingSave in a replay buffer. You probably don't want this.
///
/// Will resize recording_save.
pub fn overwrite_recsave(replay_buffer: &mut Vec<u8>, recording_save: &mut Vec<u8>) {
    let recording_offset = u32::from_be_bytes(replay_buffer[60..64].try_into().unwrap()) as usize;
    let menu_offset = u32::from_be_bytes(replay_buffer[64..68].try_into().unwrap()) as usize;

    let menu_settings: [u8; 6] = replay_buffer[menu_offset..][..6].try_into().unwrap();

    recording_save.resize(RECORDING_SIZE + 257, 0u8);

    // compress
    replay_buffer.resize(recording_offset + RECORDING_SIZE, 0u8);
    let recording_compressed_size = compress::lz77_compress(
        &recording_save, 
        RECORDING_SIZE as u32, 
        &mut replay_buffer[recording_offset..]
    ) as usize;
    replay_buffer.resize(recording_offset+recording_compressed_size, 0u8);

    let new_menu_offset = replay_buffer.len();
    replay_buffer.extend_from_slice(&menu_settings);
    replay_buffer[64..68].copy_from_slice(&(new_menu_offset as u32).to_be_bytes());
}

/// Construct TM replay from a raw replay buffer. You probably don't want this.
///
/// Anatomy of a replay buffer:
/// ```c
/// struct ReplayBuffer {
///     ExportHeader header;
///     RecordingSaveCompressed recsave_compressed;
///     MenuSettings menu_settings;
/// };
/// ```
/// This struct is just provided for clarity - it is not a real struct used in Training-Mode/patch/events/lab/source/lab.c.
pub fn construct_tm_replay_from_replay_buffer(
    date: RecordingTime,
    filename: &[u8; 31],
    replay_buffer: &[u8],
) -> Result<Vec<u8>, ReplayCreationError> {
    // for the gci file
    let mut bytes = Vec::with_capacity(8096 * 8);

    bytes.extend_from_slice(DEFAULT_GCI_HEADER);

    let ident = "GTME01";

    use std::time::SystemTime;
    let rand = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize & 0xFFFFFFFF;

    bytes[0..6].copy_from_slice(ident.as_bytes());
    let gci_inner_name = format!(
        "TMREC_{:02}{:02}{:04}_{:02}{:02}{:02}_{:08x}",
        date.month, date.day, date.year,
        date.hour, date.minute, date.second,
        rand,
    );
    bytes[8..0x28].fill(0);
    bytes[8..8+gci_inner_name.len()].copy_from_slice(gci_inner_name.as_bytes());

    bytes[0x60..][..31].copy_from_slice(filename);

    assert!(bytes.len() == 0x1EB0);

    // recalculate gci header checksum

    let (gci_header, checksum) = bytes.split_at_mut(0x1E80);
    calculate_checksum(&gci_header[0x40..], checksum);

    // round up division by block size
    // subtract 400 bytes, because first block always has that size for some reason
    let full_blocks = (replay_buffer.len() - (400-32) + (BLOCK_SIZE-32) - 1) / (BLOCK_SIZE-32);

    let mut block_header = WEIRD_BLOCK_HEADER;
    block_header[5..7].copy_from_slice(&(replay_buffer.len() as u16).to_be_bytes()); // write size
    
    bytes.resize(bytes.len() + 16, 0); // space for checksum
    bytes.extend_from_slice(&block_header);
    bytes.extend_from_slice(&replay_buffer[0..(400 - 32)]);

    for i in 0..full_blocks {
        bytes.resize(bytes.len() + 16, 0); // space for checksum
        bytes.extend_from_slice(&block_header);
        let len = bytes.len();
        bytes[len-16..][..2].copy_from_slice(&(i as u16 + 1).to_be_bytes()); // block idx

        let block_data_start = (400-32) + (BLOCK_SIZE-32)*i;
        if replay_buffer[block_data_start..].len() >= (BLOCK_SIZE-32) {
            bytes.extend_from_slice(&replay_buffer[block_data_start..block_data_start+BLOCK_SIZE-32]);
        } else {
            bytes.extend_from_slice(&replay_buffer[block_data_start..]);
        }
    }

    // fill out last block
    bytes.resize(0x1EB0 + 400 + BLOCK_SIZE*full_blocks, 0u8);

    encode_block(&mut bytes[0x1EB0..0x1EB0+400]);

    for i in 0..full_blocks {
        let start = 0x1EB0 + 400 + BLOCK_SIZE*i;
        encode_block(&mut bytes[start..start+BLOCK_SIZE]);
    }

    Ok(bytes)
}

/// Construct TM replay from initial state and inputs.
/// See `construct_tm_replay_from_slp` for more details.
pub fn construct_tm_replay(
    state: &RecordingState, 
    inputs: &InputRecordings,
    flags: ReplayFlags,
) -> Result<Vec<u8>, ReplayCreationError> {
    if state.cpu_state.character.character() == slp_parser::Character::Zelda { 
        return Err(ReplayCreationError::ZeldaOnCpu) 
    }

    //if let slp_parser::ActionState::Special(_) = state.hmn_state.state {
    //    return Err(ReplayCreationError::SpecialActionState)
    //}

    //if let slp_parser::ActionState::Special(_) = state.cpu_state.state {
    //    return Err(ReplayCreationError::SpecialActionState)
    //}

    // buffer created by unclepunch's tm code
    let mut bytes = Vec::with_capacity(8192 * 8);

    state.write_header(&mut bytes, flags & replay_flags::SWAP_SHEIK_ZELDA != 0);

    //let mut image = include_bytes!("/home/alex/Downloads/test_image_rgb565.bin");
    //let mut image = [0u8; 2*96*72];
    //for i in 0..image.len() {
    //    let b1 = 0b11010111;
    //    image[i] = b1;
    //}

    let screenshot_offset = bytes.len();
    let screenshot_size = 2 * 96 * 72;
    bytes.resize(68 + screenshot_size, 0u8); // black screen for now
    //tes.extend_from_slice(image);

    let recording_offset = bytes.len();

    let mut recording_save = vec![0u8; RECORDING_SIZE + 257]; // pad a bit for compression algo
    let rec_start = SAVESTATE_SIZE+MATCHINIT_SIZE;
    recording_save[0..rec_start].copy_from_slice(&DEFAULT_SAVESTATE_AND_MATCHINIT[..rec_start]);

    let savestate_offset = MATCHINIT_SIZE;
    recording_save[savestate_offset+4..][..4].copy_from_slice(&state.start_frame.to_be_bytes());

    // overwrite MatchInit values

    let stage = state.stage.to_u16_external();
    recording_save[0x0E..][..2].copy_from_slice(&stage.to_be_bytes());

    // write FtState values

    fn write_ft_state(ft_state: &mut [u8], st: &CharacterState, follower: Option<&CharacterState>) {
        let ft_savestate_data_size = 4396;
        let playerblock_offset = ft_savestate_data_size*2;
        let stale_offset = 8972;

        write_ft_save_state_data(ft_state, st);
        if let Some(follower_st) = follower { 
            write_ft_save_state_data(&mut ft_state[ft_savestate_data_size..], follower_st);
        }

        // stale moves ------------------------------------

        let stale_move_next_idx = st.stale_moves.iter()
            .position(|st| st.attack == slp_parser::AttackKind::Null)
            .unwrap_or(0) as u32;
        ft_state[stale_offset..][..4].copy_from_slice(&stale_move_next_idx.to_be_bytes());

        for i in 0..10 {
            let offset = stale_offset + 4 + 4*i;
            let st = st.stale_moves[i];
            ft_state[offset+1..][..1].copy_from_slice(&(st.attack as u8).to_be_bytes());
            ft_state[offset+2..][..2].copy_from_slice(&st.instance_id.to_be_bytes());
        }

        // Playerblock ---------------------------------

        // fix stock icons
        let character = st.character.character().to_u8_external().unwrap();
        let costume = st.character.costume_idx();
        ft_state[playerblock_offset..][4..8].copy_from_slice(&(character as u32).to_be_bytes());
        ft_state[playerblock_offset..][68] = costume;
    }
    
    fn write_ft_save_state_data(ft_state: &mut [u8], st: &CharacterState) {
        // nested struct offsets
        let phys_offset = 40;
        let input_offset = 568;
        let collision_offset = 676; // CollData
        let camera_box_offset = 1092; // CameraBox
        let flags_offset = 3356;
        let char_fighter_var_offset = 3384;
        let char_state_var_offset = 3592;
        let subaction_flags_offset = 3664;
        let dmg_offset = 3680;
        let jump_offset = 4048;

        // state, direction, anim frame, anim speed, anim blend
        let state_offset = 4;
        ft_state[state_offset..][..4].copy_from_slice(&(st.state.as_u16() as u32).to_be_bytes());
        let direction_bytes = match st.direction {
            slp_parser::Direction::Left => (-1.0f32).to_be_bytes(),
            slp_parser::Direction::Right => 1.0f32.to_be_bytes(),
        };
        ft_state[state_offset..][4..8].copy_from_slice(&direction_bytes);
        ft_state[state_offset..][8..12].copy_from_slice(&st.state_frame.to_be_bytes());
        ft_state[state_offset..][12..16].copy_from_slice(&st.state_speed.to_be_bytes());
        ft_state[state_offset..][16..20].copy_from_slice(&st.state_blend.to_be_bytes());

        // idk
        ft_state[state_offset..][20..24].copy_from_slice(&(st.x_rotn_rot[0]).to_be_bytes());
        ft_state[state_offset..][24..28].copy_from_slice(&(st.x_rotn_rot[1]).to_be_bytes());
        ft_state[state_offset..][28..32].copy_from_slice(&(st.x_rotn_rot[2]).to_be_bytes());
        ft_state[state_offset..][32..36].copy_from_slice(&(st.x_rotn_rot[3]).to_be_bytes());

        // phys struct -------------------------
        
        // velocities
        ft_state[phys_offset..][0..4].copy_from_slice(&st.anim_velocity[0].to_be_bytes()); // anim_vel.x
        ft_state[phys_offset..][4..8].copy_from_slice(&st.anim_velocity[1].to_be_bytes()); // anim_vel.y
        ft_state[phys_offset..][8..12].copy_from_slice(&st.anim_velocity[2].to_be_bytes()); // anim_vel.z
        ft_state[phys_offset..][12..16].copy_from_slice(&st.self_velocity[0].to_be_bytes()); // self_vel.x
        ft_state[phys_offset..][16..20].copy_from_slice(&st.self_velocity[1].to_be_bytes()); // self_vel.y
        ft_state[phys_offset..][20..24].copy_from_slice(&st.self_velocity[2].to_be_bytes()); // self_vel.z
        ft_state[phys_offset..][24..28].copy_from_slice(&st.hit_velocity[0].to_be_bytes()); // kb_vel.x
        ft_state[phys_offset..][28..32].copy_from_slice(&st.hit_velocity[1].to_be_bytes()); // kb_vel.y
        ft_state[phys_offset..][32..36].copy_from_slice(&st.hit_velocity[2].to_be_bytes()); // kb_vel.z
        ft_state[phys_offset..][120..124].copy_from_slice(&st.ground_velocity[0].to_be_bytes()); // selfVelGround.x
        ft_state[phys_offset..][124..128].copy_from_slice(&st.ground_velocity[1].to_be_bytes()); // selfVelGround.y
        ft_state[phys_offset..][128..132].copy_from_slice(&st.ground_velocity[2].to_be_bytes()); // selfVelGround.z

        // position
        ft_state[phys_offset..][60..64].copy_from_slice(&st.position[0].to_be_bytes()); // pos.x
        ft_state[phys_offset..][64..68].copy_from_slice(&st.position[1].to_be_bytes()); // pos.y
        ft_state[phys_offset..][68..72].copy_from_slice(&st.position[2].to_be_bytes()); // pos.z
        ft_state[phys_offset..][72..76].copy_from_slice(&st.prev_position[0].to_be_bytes()); // pos_prev.x
        ft_state[phys_offset..][76..80].copy_from_slice(&st.prev_position[1].to_be_bytes()); // pos_prev.y
        ft_state[phys_offset..][80..84].copy_from_slice(&st.prev_position[2].to_be_bytes()); // pos_prev.z
        ft_state[phys_offset..][84..88].copy_from_slice(&(0.0f32).to_be_bytes()); // pos_delta.x
        ft_state[phys_offset..][88..92].copy_from_slice(&(0.0f32).to_be_bytes()); // pos_delta.y
        ft_state[phys_offset..][92..96].copy_from_slice(&(0.0f32).to_be_bytes()); // pos_delta.z

        ft_state[phys_offset..][108..112].copy_from_slice(&(st.airborne as u32).to_be_bytes());
        
        // input struct -----------------

        ft_state[input_offset..][0..4].copy_from_slice(&st.stick[0].to_be_bytes());
        ft_state[input_offset..][4..8].copy_from_slice(&st.stick[1].to_be_bytes());
        ft_state[input_offset..][8..12].copy_from_slice(&st.prev_stick[0].to_be_bytes());
        ft_state[input_offset..][12..16].copy_from_slice(&st.prev_stick[1].to_be_bytes());
        ft_state[input_offset..][24..28].copy_from_slice(&st.cstick[0].to_be_bytes());
        ft_state[input_offset..][28..32].copy_from_slice(&st.cstick[1].to_be_bytes());
        ft_state[input_offset..][48..52].copy_from_slice(&st.trigger.to_be_bytes());

        ft_state[input_offset..][60..64].copy_from_slice(&(st.held as u32).to_be_bytes());
        ft_state[input_offset..][64..68].copy_from_slice(&(st.prev_held as u32).to_be_bytes());
        ft_state[input_offset..][72..76].copy_from_slice(&((st.prev_held & st.held) as u32).to_be_bytes());

        ft_state[input_offset..][0x50] = st.input_timers.timer_lstick_tilt_x;            
        ft_state[input_offset..][0x51] = st.input_timers.timer_lstick_tilt_y;            
        ft_state[input_offset..][0x52] = st.input_timers.timer_trigger_analog;           
        ft_state[input_offset..][0x53] = st.input_timers.timer_lstick_smash_x;           
        ft_state[input_offset..][0x54] = st.input_timers.timer_lstick_smash_y;           
        ft_state[input_offset..][0x55] = st.input_timers.timer_trigger_digital;          
        ft_state[input_offset..][0x56] = st.input_timers.timer_lstick_any_x;             
        ft_state[input_offset..][0x57] = st.input_timers.timer_lstick_any_y;             
        ft_state[input_offset..][0x58] = st.input_timers.timer_trigger_any;              
        ft_state[input_offset..][0x59] = st.input_timers.x679_x;                         
        ft_state[input_offset..][0x5A] = st.input_timers.x67A_y;                         
        ft_state[input_offset..][0x5B] = st.input_timers.x67B;                           
        ft_state[input_offset..][0x5C] = st.input_timers.timer_a;                        
        ft_state[input_offset..][0x5D] = st.input_timers.timer_b;                        
        ft_state[input_offset..][0x5E] = st.input_timers.timer_xy;                       
        ft_state[input_offset..][0x5F] = st.input_timers.timer_trigger_any_ignore_hitlag;
        ft_state[input_offset..][0x60] = st.input_timers.timer_LR;                       
        ft_state[input_offset..][0x61] = st.input_timers.timer_padup;                    
        ft_state[input_offset..][0x62] = st.input_timers.timer_paddown;                  
        ft_state[input_offset..][0x63] = st.input_timers.timer_item_release;             
        ft_state[input_offset..][0x64] = st.input_timers.since_rapid_lr;                 
        ft_state[input_offset..][0x65] = st.input_timers.timer_jump;                     
        ft_state[input_offset..][0x66] = st.input_timers.timer_specialhi;                
        ft_state[input_offset..][0x67] = st.input_timers.timer_speciallw;                
        ft_state[input_offset..][0x68] = st.input_timers.timer_specials;                 
        ft_state[input_offset..][0x69] = st.input_timers.timer_specialn;                 
        ft_state[input_offset..][0x6A] = st.input_timers.timer_jump_lockout;             
        ft_state[input_offset..][0x6B] = st.input_timers.timer_specialhi_lockout;        

        let percent_bytes = (st.percent*0.5).to_be_bytes(); // percent is stored halved for some reason???
        ft_state[dmg_offset..][4..8].copy_from_slice(&percent_bytes); // percent
        ft_state[dmg_offset..][12..16].copy_from_slice(&percent_bytes); // temp percent???
        ft_state[dmg_offset..][0x80..0x84].copy_from_slice(&st.frames_since_hit.to_be_bytes()); // frames in knockback
        ft_state[dmg_offset..][0xE4..0xE8].copy_from_slice(&st.offscreen_damage_timer.to_be_bytes());
        
        // collision data (CollData) ------------------

        // I believe these set the centre of the ECB.
        // topN_Curr
        ft_state[collision_offset..][4..8].copy_from_slice(&st.position[0].to_be_bytes());
        ft_state[collision_offset..][8..12].copy_from_slice(&st.position[1].to_be_bytes());
        ft_state[collision_offset..][12..16].copy_from_slice(&st.position[2].to_be_bytes());
        // topN_CurrCorrect
        ft_state[collision_offset..][16..20].copy_from_slice(&st.position[0].to_be_bytes());
        ft_state[collision_offset..][20..24].copy_from_slice(&st.position[1].to_be_bytes());
        ft_state[collision_offset..][24..28].copy_from_slice(&st.position[2].to_be_bytes());
        // topN_Prev
        ft_state[collision_offset..][28..32].copy_from_slice(&st.prev_position[0].to_be_bytes());
        ft_state[collision_offset..][32..36].copy_from_slice(&st.prev_position[1].to_be_bytes());
        ft_state[collision_offset..][36..40].copy_from_slice(&st.prev_position[2].to_be_bytes());
        // topN_Proj
        ft_state[collision_offset..][40..44].copy_from_slice(&st.position[0].to_be_bytes());
        ft_state[collision_offset..][44..48].copy_from_slice(&st.position[1].to_be_bytes());
        ft_state[collision_offset..][48..52].copy_from_slice(&st.position[2].to_be_bytes());

        if st.airborne {
            // if the character is low enough, then the ecb in the air will be below the stage and
            // the character will phase through the stage.
            // It's almost never lower than 4.0 (probably)
            ft_state[collision_offset..][176..180].copy_from_slice(&(4.0f32).to_be_bytes());
        }

        ft_state[collision_offset..][332..336].copy_from_slice(&st.last_ground_idx.to_be_bytes());

        // camera data (CameraBox) -------------------------------------
        
        ft_state[camera_box_offset..][0..4].copy_from_slice(&[0u8; 4]); // alloc
        ft_state[camera_box_offset..][4..8].copy_from_slice(&[0u8; 4]); // next box ptr
        // cam pos
        ft_state[camera_box_offset..][16..20].copy_from_slice(&st.position[0].to_be_bytes());
        ft_state[camera_box_offset..][20..24].copy_from_slice(&st.position[1].to_be_bytes());
        ft_state[camera_box_offset..][24..28].copy_from_slice(&st.position[2].to_be_bytes());
        // bone pos (necessary - causes character culling otherwise)
        ft_state[camera_box_offset..][28..32].copy_from_slice(&st.position[0].to_be_bytes());
        ft_state[camera_box_offset..][32..36].copy_from_slice(&st.position[1].to_be_bytes());
        ft_state[camera_box_offset..][36..40].copy_from_slice(&st.position[2].to_be_bytes());

        // hitlag & hitstun handling -----------------------------

        if st.hitlag_frames_left > 0.0 {
            ft_state[dmg_offset..][304..308].copy_from_slice(&st.hitlag_frames_left.to_be_bytes());
            ft_state[flags_offset..][9] = 4;  // hitstop flag
        }

        // flags ----------------------------------------------

        if matches!(
            st.state,
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::Catch | slp_parser::StandardActionState::CatchDash)
        ) {
            // if not set, grabs in progress will always whiff.
            ft_state[flags_offset..][14] = 0x1A; // 0x19 -> 0x1A
        }

        ft_state[flags_offset..][8] = st.state_flags[0];
        ft_state[flags_offset..][10] = st.state_flags[1];
        ft_state[flags_offset..][11] = st.state_flags[2];
        ft_state[flags_offset..][12] = st.state_flags[3];
        ft_state[flags_offset..][15] = st.state_flags[4];

        ft_state[flags_offset..][24] &= !1;         
        ft_state[flags_offset..][24] |= match st.last_lstick_x_direction {
            slp_parser::Direction::Left => 0,
            slp_parser::Direction::Right => 1,
        };

        // multijump flag
        if matches!(
            st.character.character(), 
            slp_parser::Character::Jigglypuff | slp_parser::Character::Kirby
        ) {
            ft_state[flags_offset..][18] |= 0x40;
        } else {
            ft_state[flags_offset..][18] &= !0x40;
        }

        // walljump flag
        if matches!(
            st.character.character(),
            slp_parser::Character::Mario 
            | slp_parser::Character::CaptainFalcon
            | slp_parser::Character::Falco
            | slp_parser::Character::Fox
            | slp_parser::Character::Samus
            | slp_parser::Character::Sheik
            | slp_parser::Character::YoungLink
            | slp_parser::Character::Pichu
        ) {
            ft_state[flags_offset..][20] |= 0x01;
        } else {
            ft_state[flags_offset..][20] &= !0x01;
        }

        ft_state[char_fighter_var_offset..][0..208].copy_from_slice(&st.char_fighter_var);
        ft_state[char_state_var_offset..][0..72].copy_from_slice(&st.char_state_var);
        ft_state[subaction_flags_offset..][0..16].copy_from_slice(&st.subaction_flags);
        
        ft_state[jump_offset..][0] = jump_count(st.character.character())- st.jumps_remaining;
        
        // callbacks (struct cb) ------------------------------

        let fns_idx = (st.state.as_u16() as usize) * 0x20;

        let fns = if fns_idx < ACTION_FN_LOOKUP_TABLE.len() {
            &ACTION_FN_LOOKUP_TABLE[fns_idx..][..0x20]
        } else {
            let c = st.character.character().to_u8_internal() as usize;
            let offset = SPECIAL_ACTION_FN_CHARACTER_OFFSETS[c] as usize;
            let special_fns_idx = offset + (fns_idx - ACTION_FN_LOOKUP_TABLE.len());
            &SPECIAL_ACTION_FN_LOOKUP_TABLE[special_fns_idx..][..0x20]
        };

        ft_state[0x10CC..][0..4].copy_from_slice(&fns[16..20]); // IASA
        ft_state[0x10CC..][4..8].copy_from_slice(&fns[12..16]); // Anim
        ft_state[0x10CC..][8..20].copy_from_slice(&fns[20..32]); // Phys, Coll, Cam
    }

    let st_offset = 312; // savestate offset - skip MatchInit in RecordingSave
    let ft_state_offset = 8+EVENT_DATASIZE; // FtState array offset - fields in Savestate;
    let ft_state_size = 9016;
    write_ft_state(
        &mut recording_save[st_offset+ft_state_offset..][..ft_state_size],
        &state.hmn_state,
        state.hmn_follower_state.as_ref(),
    );
    write_ft_state(
        &mut recording_save[st_offset+ft_state_offset+ft_state_size..][..ft_state_size],
        &state.cpu_state,
        state.cpu_follower_state.as_ref(),
    );

    // write inputs

    fn write_inputs(slot: &mut [u8], start_frame: i32, inputs: Option<&[Input]>) -> Result<(), ReplayCreationError> {
        if let Some(i) = inputs { 
            if i.len() > 3600 { return Err(ReplayCreationError::DurationTooLong) } 
        }

        // if None or len == 0
        if !inputs.is_some_and(|i| !i.is_empty()) {
            slot[0..4].copy_from_slice(&(-1i32).to_be_bytes()); // start_frame
            slot[4..8].copy_from_slice(&0u32.to_be_bytes());    // num_frames
            return Ok(());
        }

        let inputs = inputs.unwrap();

        slot[0..4].copy_from_slice(&start_frame.to_be_bytes()); // start frame
        slot[4..8].copy_from_slice(&(inputs.len() as u32).to_be_bytes());    // num_frames

        for frame in 0..inputs.len() {
            let offset = 8 + frame*6;
            let input = inputs[frame];

            slot[offset..offset+6].copy_from_slice(&[
                input.button_flags,
                input.stick_x as u8,
                input.stick_y as u8,
                input.cstick_x as u8,
                input.cstick_y as u8,
                input.trigger,
            ]);
        }

        Ok(())
    }

    // hmn inputs
    for i in 0..REC_SLOTS {
        let input_data_start = rec_start + i*REC_SLOT_SIZE;
        let slot = &mut recording_save[input_data_start..][..REC_SLOT_SIZE];
        write_inputs(slot, state.start_frame, inputs.hmn_slots[i])?;
    }

    // cpu inputs
    for i in 0..REC_SLOTS {
        let input_data_start = rec_start + (i+6)*REC_SLOT_SIZE;
        let slot = &mut recording_save[input_data_start..][..REC_SLOT_SIZE];
        write_inputs(slot, state.start_frame, inputs.cpu_slots[i])?;
    }

    // compress
    bytes.resize(recording_offset + RECORDING_SIZE, 0u8);
    let recording_compressed_size = compress::lz77_compress(
        &recording_save, 
        RECORDING_SIZE as u32, 
        &mut bytes[recording_offset..]
    ) as usize;
    bytes.resize(recording_offset+recording_compressed_size, 0u8);

    let menu_settings_offset = bytes.len();

    state.write_menu_settings(&mut bytes);

    bytes[56..60].copy_from_slice(&(screenshot_offset as u32).to_be_bytes());
    bytes[60..64].copy_from_slice(&(recording_offset as u32).to_be_bytes());
    bytes[64..68].copy_from_slice(&(menu_settings_offset as u32).to_be_bytes());

    construct_tm_replay_from_replay_buffer(state.time, &state.filename, &bytes)
}

#[derive(Copy, Clone, PartialEq)]
pub enum HumanPort {
    HumanLowPort,
    HumanHighPort,
}

pub type ReplayFlags = u64;
pub mod replay_flags {
    use super::ReplayFlags;
    pub const SWAP_SHEIK_ZELDA: ReplayFlags = 1 << 0;
}


/// Construct TM replay from slp file.
///
/// Returns GCI file bytes.
///
/// # Unimplemented
/// - items
/// - animation blending
///
/// # Errors
/// - If duration is greater than 3600 frames
/// - If name is longer than 31 bytes
/// - If name is not ASCII
/// - If either character is in a special action state (will be supported in the future)
/// - If Zelda is on cpu. This is due to a bug in Unclepunch.
pub fn construct_tm_replay_from_slp(
    game: &slp_parser::Game, 
    human: HumanPort,
    frame: usize,
    duration: usize,
    name: &str,
    flags: ReplayFlags,
) -> Result<Vec<u8>, ReplayCreationError> {
    let major = game.info.version_major;
    let minor = game.info.version_minor;
    if major < MIN_VERSION_MAJOR || (major == MIN_VERSION_MAJOR && minor < MIN_VERSION_MINOR) {
        return Err(ReplayCreationError::OutdatedReplay);
    }
    
    let mut frame = frame;
    let mut duration = duration;

    let (low_port, high_port) = match game.info.low_high_ports() {
        Some(p) => p,
        None => return Err(ReplayCreationError::NotTwoPlayerGame),
    };
    let low_port_frames = game.frames[low_port].as_ref().unwrap();
    let high_port_frames = game.frames[high_port].as_ref().unwrap();
    let low_follower_frames = game.follower_frames[low_port].as_ref();
    let high_follower_frames = game.follower_frames[high_port].as_ref();
    let low_starting_character = game.info.starting_character_colours[low_port].unwrap();
    let high_starting_character = game.info.starting_character_colours[high_port].unwrap();

    let mut frames = Vec::with_capacity(4);
    frames.push(low_port_frames);
    frames.push(high_port_frames);
    if let Some(low_follower_frames) = low_follower_frames {
        frames.push(low_follower_frames);
    }
    if let Some(high_follower_frames) = high_follower_frames {
        frames.push(high_follower_frames);
    }

    // search backwards for a good frame to export -------------------------

    fn good_frame(f: &slp_parser::Frame) -> bool {
        use slp_parser::{ActionState, StandardActionState::*};

        if f.hitlag_frames != 0.0 { return false }

        if matches!(f.state, ActionState::Standard(
            CatchPull | CatchDashPull | CatchWait | CatchAttack | CatchCut
                | ThrowF | ThrowB | ThrowHi | ThrowLw 
                | CapturePulledHi | CaptureWaitHi | CaptureDamageHi | CapturePulledLw | CaptureWaitLw
                | CaptureDamageLw | CaptureCut | CaptureJump | CaptureNeck | CaptureFoot
                | ThrownF | ThrownB | ThrownHi | ThrownLw | ThrownLwWomen
                | ShoulderedWait | ShoulderedWalkSlow | ShoulderedWalkMiddle | ShoulderedWalkFast | ShoulderedTurn
                | ThrownFF | ThrownFB | ThrownFHi | ThrownFLw
                | CaptureCaptain | CaptureYoshi | YoshiEgg | CaptureKoopa
                | CaptureDamageKoopa | CaptureWaitKoopa | ThrownKoopaF | ThrownKoopaB
                | CaptureKoopaAir | CaptureDamageKoopaAir | CaptureWaitKoopaAir | ThrownKoopaAirF | ThrownKoopaAirB
                | CaptureKirby | CaptureWaitKirby | ThrownKirbyStar | ThrownCopyStar | ThrownKirby
                | BarrelWait | Bury | BuryWait | BuryJump
                | DamageSong | DamageSongWait | DamageSongRv | DamageBind
        )) {
            return false;
        }
        
        let state_num = f.state_num as usize;
        if hitboxes::ATTACK_RANGE_START <= state_num && state_num < hitboxes::ATTACK_RANGE_END {
            let hitbox_range = &hitboxes::ATTACK_HITBOXES[f.character as usize][state_num];
            if hitbox_range.contains(&(f.anim_frame as u32)) {
                return false;
            }
        } 
        
        true
    }

    while frames.iter().any(|f| !good_frame(&f[frame])) {
        if frame == 0 { return Err(ReplayCreationError::NoGoodExportFrame); }
        frame -= 1;
        duration += 1;
    }

    // We need to search forwards for entry
    while matches!(
        low_port_frames[frame].state, 
        slp_parser::ActionState::Standard(slp_parser::StandardActionState::Entry
            | slp_parser::StandardActionState::EntryStart
            | slp_parser::StandardActionState::EntryEnd)
    ) || matches!(
        high_port_frames[frame].state, 
        slp_parser::ActionState::Standard(slp_parser::StandardActionState::Entry
            | slp_parser::StandardActionState::EntryStart
            | slp_parser::StandardActionState::EntryEnd)
    ) {
        frame += 1;
    }

    // export ---------------------------------------------------------------

    if frame + duration >= low_port_frames.len() {
        duration = low_port_frames.len() - frame;
    }

    if name.len() >= 32 { return Err(ReplayCreationError::FilenameTooLong) }
    if !name.is_ascii() { return Err(ReplayCreationError::FilenameNotASCII) }
    if duration > 3600 { return Err(ReplayCreationError::DurationTooLong) }

    let info = &game.info;
    let time = info.start_time.fields();

    let mut filename = [0u8; 31];
    filename[..name.len()].copy_from_slice(name.as_bytes());

    fn inputs_over_frames(frames: &[slp_parser::Frame]) -> Vec<Input> {
        frames
            .iter()
            .map(|f| {
                Input {
                    button_flags: translate_buttons_from_slp(f.buttons_mask),
                    stick_x: (f.left_stick_coords.x * 80.0) as i8,
                    stick_y: (f.left_stick_coords.y * 80.0) as i8,
                    cstick_x: (f.right_stick_coords.x * 80.0) as i8,
                    cstick_y: (f.right_stick_coords.y * 80.0) as i8,
                    trigger: (f.analog_trigger_value * 140.0) as u8,
                }
            }).collect()
    }

    fn state(
        starting_char: slp_parser::CharacterColour, 
        frames: &[slp_parser::Frame],
        opponent_frames: &[slp_parser::Frame],
        frame_idx: usize,
    ) -> CharacterState {
        let frame = &frames[frame_idx];
        let prev_frame = if frame_idx != 0 { Some(&frames[frame_idx - 1]) } else { None };
        let next_frame = frames.get(frame_idx+1);

        let prev_position;
        let prev_stick;
        let prev_held;
        match prev_frame {
            Some(p) => {
                prev_position = [p.position.x, p.position.y, 0.0];
                prev_stick = vector_to_arr(p.left_stick_coords);
                prev_held = p.buttons_mask;
            }
            None => {
                prev_position = [frame.position.x, frame.position.y, 0.0];
                prev_stick = [0.0, 0.0];
                prev_held = 0;
            }
        }

        // not recorded in slp - manually calculated
        let state_speed;
        match (prev_frame, next_frame) {
            (Some(p), _) if p.state == frame.state && p.anim_frame < frame.anim_frame => {
                state_speed = frame.anim_frame - p.anim_frame;
            }

            (_, Some(n)) if n.state == frame.state && n.anim_frame > frame.anim_frame => {
                state_speed = n.anim_frame - frame.anim_frame;
            }

            // nothing we can do here
            _ => state_speed = 1.0,
        }

        let mut char_fighter_var = [0u8; 208];

        let mut subaction_flags = [0u8; 16];
        let lag_windows = &autocancel::AERIAL_LAG_WINDOWS[frame.character as usize];

        let mut char_state_var = [0u8; 72];
        char_state_var[0..4].copy_from_slice(&frame.hitstun_misc.to_be_bytes());

        // only first char state var is recorded. The rest we have to calculate, if they matter.
        match frame.state {
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::AttackAirN ) => {
                if lag_windows.nair.contains(&(frame.anim_frame as u32)) { subaction_flags[3] = 1; }
            },
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::AttackAirF ) => {
                if lag_windows.fair.contains(&(frame.anim_frame as u32)) { subaction_flags[3] = 1; }
            },
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::AttackAirB ) => {
                if lag_windows.bair.contains(&(frame.anim_frame as u32)) { subaction_flags[3] = 1; }
            },
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::AttackAirHi) => {
                if lag_windows.uair.contains(&(frame.anim_frame as u32)) { subaction_flags[3] = 1; }
            },
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::AttackAirLw) => {
                if lag_windows.dair.contains(&(frame.anim_frame as u32)) { subaction_flags[3] = 1; }
            },

            slp_parser::ActionState::Standard(
                slp_parser::StandardActionState::JumpF
                | slp_parser::StandardActionState::JumpB
            ) => {
                char_state_var[4..8].copy_from_slice(&1u32.to_be_bytes());
            },

            slp_parser::ActionState::Standard(slp_parser::StandardActionState::KneeBend) => {
                char_state_var[4..8].copy_from_slice(&3u32.to_be_bytes());
            }

            slp_parser::ActionState::Standard(slp_parser::StandardActionState::Turn) => {
                let dir = match frame.direction {
                    slp_parser::Direction::Left => 1.0f32,
                    slp_parser::Direction::Right => -1.0f32,
                };
                char_state_var[4..8].copy_from_slice(&dir.to_be_bytes());
            }
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::CliffWait) => {
                // prevents immediate fall from ledge
                char_state_var[4..8].copy_from_slice(&640.0f32.to_be_bytes());

                // prevents other character from grabbing ledge
                char_state_var[8..12].copy_from_slice(&1u32.to_be_bytes());
                char_state_var[12..16].copy_from_slice(&40.0f32.to_be_bytes());
            }

            slp_parser::ActionState::Special(slp_parser::SpecialActionState::Marth(
                slp_parser::SpecialActionStateMarth::DolphinSlashGround
                | slp_parser::SpecialActionStateMarth::DolphinSlashAir
            )) => {
                if frame.anim_frame >= 6.0 { subaction_flags[3] = 1; }
            }

            slp_parser::ActionState::Special(slp_parser::SpecialActionState::Roy(
                slp_parser::SpecialActionStateRoy::BlazerGround
                | slp_parser::SpecialActionStateRoy::BlazerAir
            )) => {
                if frame.anim_frame >= 10.0 { subaction_flags[3] = 1; }
            }
            
            slp_parser::ActionState::Special(
                slp_parser::SpecialActionState::Jigglypuff(
                    slp_parser::SpecialActionStateJigglypuff::Jump2
                    | slp_parser::SpecialActionStateJigglypuff::Jump3
                    | slp_parser::SpecialActionStateJigglypuff::Jump4
                    | slp_parser::SpecialActionStateJigglypuff::Jump5
                ) | slp_parser::SpecialActionState::Kirby(
                    slp_parser::SpecialActionStateKirby::Jump2
                    | slp_parser::SpecialActionStateKirby::Jump3
                    | slp_parser::SpecialActionStateKirby::Jump4
                    | slp_parser::SpecialActionStateKirby::Jump5
                )
            ) => subaction_flags[3] = if frame.anim_frame >= 28.0 { 1 } else { 0 },

            slp_parser::ActionState::Special(slp_parser::SpecialActionState::Peach(
                slp_parser::SpecialActionStatePeach::Float
                | slp_parser::SpecialActionStatePeach::FloatNair
                | slp_parser::SpecialActionStatePeach::FloatFair
                | slp_parser::SpecialActionStatePeach::FloatBair
                | slp_parser::SpecialActionStatePeach::FloatUair
                | slp_parser::SpecialActionStatePeach::FloatDair
            )) => {
                let mut first_float_frame = frame_idx - 1;
                while first_float_frame > 0 {
                    if matches!(frames[first_float_frame].state,
                        slp_parser::ActionState::Special(slp_parser::SpecialActionState::Peach(
                            slp_parser::SpecialActionStatePeach::Float
                            | slp_parser::SpecialActionStatePeach::FloatNair
                            | slp_parser::SpecialActionStatePeach::FloatFair
                            | slp_parser::SpecialActionStatePeach::FloatBair
                            | slp_parser::SpecialActionStatePeach::FloatUair
                            | slp_parser::SpecialActionStatePeach::FloatDair))
                    ) {
                        first_float_frame -= 1;
                    } else {
                        first_float_frame += 1;
                        break;
                    }
                }

                let float_frames_left = 150 - (frame_idx - first_float_frame);
                char_fighter_var[4..8].copy_from_slice(&(float_frames_left as f32).to_be_bytes());
            }
            _ => (),
        }

        let frames_since_hit = match frame.state {
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::DamageHi1        
                | slp_parser::StandardActionState::DamageHi2    
                | slp_parser::StandardActionState::DamageHi3    
                | slp_parser::StandardActionState::DamageN1     
                | slp_parser::StandardActionState::DamageN2     
                | slp_parser::StandardActionState::DamageN3     
                | slp_parser::StandardActionState::DamageLw1    
                | slp_parser::StandardActionState::DamageLw2    
                | slp_parser::StandardActionState::DamageLw3    
                | slp_parser::StandardActionState::DamageAir1   
                | slp_parser::StandardActionState::DamageAir2   
                | slp_parser::StandardActionState::DamageAir3   
                | slp_parser::StandardActionState::DamageFlyHi  
                | slp_parser::StandardActionState::DamageFlyN   
                | slp_parser::StandardActionState::DamageFlyLw  
                | slp_parser::StandardActionState::DamageFlyTop 
                | slp_parser::StandardActionState::DamageFlyRoll)
            => frames[..frame_idx].iter().rev().position(|f| f.hitlag_frames != 0.0).unwrap() as i32,
            _ => -1 
        };
        
        let stale_moves = slp_parser::compute_staled_moves(
            &frames[..=frame_idx],
            &opponent_frames[..=frame_idx],
        );

        let mut offscreen_damage_timer = 0;
        let mut i = frame_idx;
        loop {
            if frames[i].state_flags[4] & 0x80 == 0 { break; }
            if i == 0 { break; }
            i -= 1;
            offscreen_damage_timer += 1;
            if offscreen_damage_timer == 60 { offscreen_damage_timer = 0; }
        }

        let mut input_timers = InputTimers::default();
        if frame_idx != 0 {
            // 60 frames to comput timers should be enough.
            let mut min_frame = frame_idx.saturating_sub(60) + 1;

            for i in min_frame..=frame_idx {
                if frames[i].state.broad_state() == slp_parser::BroadState::Standard(slp_parser::StandardBroadState::Dead) {
                    min_frame = i;
                }
            }

            for i in min_frame..=frame_idx {
                input_timers.advance(&frames[i], &frames[i-1]);
            }
        }

        let mut last_lstick_x_direction = slp_parser::Direction::Left;
        for f in frames[..=i].iter().rev() {
            if f.left_stick_coords.x < 0.0 {
                last_lstick_x_direction = slp_parser::Direction::Left;
                break;
            } else if f.left_stick_coords.x > 0.0 {
                last_lstick_x_direction = slp_parser::Direction::Right;
                break;
            }
        }

        CharacterState {
            // respect zelda/sheik transformation
            character: slp_parser::CharacterColour::from_character_and_colour(
               frame.character, 
               starting_char.costume_idx()
            ).unwrap(),
            position: [frame.position.x, frame.position.y, 0.0],
            airborne: frame.is_airborne,
            last_ground_idx: frame.last_ground_idx as u32,
            state: frame.state,
            state_frame: frame.anim_frame,
            state_speed,
            direction: frame.direction,
            percent: frame.percent,
            self_velocity: [frame.velocity.x, frame.velocity.y, 0.0],
            hit_velocity: [frame.hit_velocity.x, frame.hit_velocity.y, 0.0],
            ground_velocity: [frame.ground_x_velocity, 0.0, 0.0],
            frames_since_hit,
            char_fighter_var,
            char_state_var,
            jumps_remaining: frame.jumps_remaining,
            hitlag_frames_left: frame.hitlag_frames,
            subaction_flags,
            state_flags: frame.state_flags,
            stale_moves,
            offscreen_damage_timer,

            prev_position,
            prev_stick,
            prev_held,
            stick: vector_to_arr(frame.left_stick_coords),
            cstick: vector_to_arr(frame.right_stick_coords),
            held: frame.buttons_mask,
            trigger: frame.analog_trigger_value,
            last_lstick_x_direction,
            input_timers,

            // state_blend, x_rotn_rot, anim_velocity
            ..Default::default()
        }
    }

    let low_state = state(low_starting_character, low_port_frames, high_port_frames, frame);
    let high_state = state(high_starting_character, high_port_frames, low_port_frames, frame);
    let low_follower_state = low_follower_frames
        .map(|f| state(low_starting_character, f, high_port_frames, frame));
    let high_follower_state = high_follower_frames
        .map(|f| state(high_starting_character, f, low_port_frames, frame));

    let hmn_frames;
    let hmn_state;
    let hmn_follower_state;
    let cpu_frames;
    let cpu_state;
    let cpu_follower_state;
    match human {
        HumanPort::HumanLowPort => {
            hmn_frames = low_port_frames;
            cpu_frames = high_port_frames;
            hmn_state = low_state;
            cpu_state = high_state;
            hmn_follower_state = low_follower_state;
            cpu_follower_state = high_follower_state;
        },
        HumanPort::HumanHighPort => {
            cpu_frames = low_port_frames;
            hmn_frames = high_port_frames;
            cpu_state = low_state;
            hmn_state = high_state;
            cpu_follower_state = low_follower_state;
            hmn_follower_state = high_follower_state;
        },
    };

    let inputs_range = if duration == 0 {
        frame..frame
    } else {
        frame+1..frame+duration
    };

    construct_tm_replay(
        &RecordingState {
            stage: info.stage,
            time: RecordingTime {
                year: time.year,
                month: time.month,
                day: time.day,
                hour: time.hour,
                minute: time.minute,
                second: time.second,
            },
            filename,
            menu_settings: RecordingMenuSettings {
                hmn_mode: HmnRecordingMode::Playback,
                hmn_slot: RecordingSlot::Slot1,
                cpu_mode: CpuRecordingMode::Playback,
                cpu_slot: RecordingSlot::Slot1,
                ..Default::default()
            },

            start_frame: (frame as i32) - 123, // start at - 123
            hmn_state,
            hmn_follower_state,
            cpu_state,
            cpu_follower_state,
        },
        &InputRecordings {
            hmn_slots: [
                Some(&inputs_over_frames(&hmn_frames[inputs_range.clone()])),
                None, None, None, None, None
            ],
            cpu_slots: [
                Some(&inputs_over_frames(&cpu_frames[inputs_range.clone()])),
                None, None, None, None, None
            ],
        },
        flags
    )
}
