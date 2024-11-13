// pub mod gen;
mod compress;

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

#[derive(Copy, Clone, Debug)]
pub struct StaleMove {
    pub move_id: u16,
    pub instance_id: u16,
}

impl StaleMove {
    pub const NULL: StaleMove = StaleMove { move_id: 0, instance_id: 0 };
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
    /// Name to show when browsing in Training Mode.
    pub filename: [u8; 31], // ascii
    pub menu_settings: RecordingMenuSettings,

    /// Melee starts at frame -123. 'GO' disappears on frame 0.
    pub start_frame: i32,
    pub stage: slp_parser::Stage,
    pub hmn_state: CharacterState,
    pub cpu_state: CharacterState,
}

impl RecordingState {
    // offsets zeroed but not written
    fn write_header(&self, b: &mut Vec<u8>) {
        // We swap zelda and shiek to work around bugs in Unclepunch.
        // This is somehow the only way to get the correct character on hmn.
        // Zelda on cpu always turns into sheik for some reason.
        //
        // The default savestate match init was generated with two sheiks on FD.
        // Other character combinations tend to crash with zelda and sheik 
        // or have the unused transformation tpose in the centre
        // This combination doesn't seem to have these issues, but it prevents using zelda on cpu.
        let char_hmn = match self.hmn_state.character.character() {
            slp_parser::Character::Zelda => slp_parser::Character::Sheik,
            slp_parser::Character::Sheik => slp_parser::Character::Zelda,
            c => c,
        }.to_u8_external().unwrap();

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
    /// Zero if n/a. See https://docs.google.com/spreadsheets/d/1spibzWaitiA22s7db1AEw1hqQXzPDNFZHYjc4czv2dc
    pub stale_moves: [StaleMove; 10],
    pub anim_velocity: [f32; 3],
    pub self_velocity: [f32; 3],
    pub hit_velocity: [f32; 3],
    pub ground_velocity: [f32; 3],

    /// number of frames in knockback if in knockback, otherwise -1
    pub frames_since_hit: i32,

    /// Generic character state variables, used for most actions.
    ///
    /// ## item 0:
    /// - During hitstun and hitstop: the number of frames of hitstun remaining
    /// - During turn: set to 1 when actionable (???)
    ///
    /// ## item 1:
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
    pub trigger: f32,
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
            percent: 0.0,
            stale_moves: [StaleMove::NULL; 10],
            anim_velocity: [0.0; 3],
            self_velocity: [0.0; 3],
            hit_velocity: [0.0; 3],
            ground_velocity: [0.0; 3],
            frames_since_hit: -1,
            hitlag_frames_left: 0.0,
            char_state_var: [0u8; 72],
            subaction_flags: [0u8; 16],
            prev_position: [0.0; 3],
            stick: [0.0; 2],
            cstick: [0.0; 2],
            prev_stick: [0.0; 2],
            state_flags: [0; 5],
            trigger: 0.0,
            state_speed: 1.0,
            state_blend: 0.0,
            x_rotn_rot: [0.0, 0.0, 0.0, 0.0],
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

    state.write_header(&mut bytes);

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
    
    fn write_ft_state(ft_state: &mut [u8], st: &CharacterState) {
        // nested struct offsets
        let phys_offset = 40;
        let input_offset = 568;
        let collision_offset = 676; // CollData
        let camera_box_offset = 1092; // CameraBox
        let flags_offset = 3356;
        let char_state_offset = 3592;
        let subaction_flags_offset = 3664;
        let dmg_offset = 3680;
        let playerblock_offset = 4396*2;
        let stale_offset = 8972;

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

        let percent_bytes = (st.percent*0.5).to_be_bytes(); // percent is stored halved for some reason???
        ft_state[dmg_offset..][4..8].copy_from_slice(&percent_bytes); // percent
        ft_state[dmg_offset..][12..16].copy_from_slice(&percent_bytes); // temp percent???
        ft_state[dmg_offset..][0x80..0x84].copy_from_slice(&st.frames_since_hit.to_be_bytes()); // frames in knockback
        
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

        ft_state[char_state_offset..][0..72].copy_from_slice(&st.char_state_var);
        ft_state[subaction_flags_offset..][0..16].copy_from_slice(&st.subaction_flags);

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

        // stale moves ------------------------------------

        let stale_move_next_idx = st.stale_moves.iter().position(|st| st.move_id == 0).unwrap_or(0) as u32;
        ft_state[stale_offset..][..4].copy_from_slice(&stale_move_next_idx.to_be_bytes());

        for i in 0..10 {
            let offset = stale_offset + 4 + 4*i;
            let st = st.stale_moves[i];
            ft_state[offset..][..2].copy_from_slice(&st.move_id.to_be_bytes());
            ft_state[offset+2..][..2].copy_from_slice(&st.instance_id.to_be_bytes());
        }

        // Playerblock ---------------------------------

        // fix stock icons
        let character = st.character.character().to_u8_external().unwrap();
        let costume = st.character.costume_idx();
        ft_state[playerblock_offset..][4..8].copy_from_slice(&(character as u32).to_be_bytes());
        ft_state[playerblock_offset..][68] = costume;
    }

    let st_offset = 312; // savestate offset - skip MatchInit in RecordingSave
    let ft_state_offset = 8+EVENT_DATASIZE; // FtState array offset - fields in Savestate;
    let ft_state_size = 9016;
    write_ft_state(&mut recording_save[st_offset+ft_state_offset..][..ft_state_size], &state.hmn_state);
    write_ft_state(&mut recording_save[st_offset+ft_state_offset+ft_state_size..][..ft_state_size], &state.cpu_state);

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

/// Construct TM replay from slp file.
///
/// Returns GCI file bytes.
///
/// # Unimplemented
/// - stale moves
/// - items
/// - animation blending
///
/// # Errors
/// - If frame + duration is out of bounds
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
) -> Result<Vec<u8>, ReplayCreationError> {
    let mut frame = frame;
    let mut duration = duration;

    let (low_port, high_port) = match game.low_high_ports() {
        Some(p) => p,
        None => return Err(ReplayCreationError::NotTwoPlayerGame),
    };
    let low_port_frames = game.frames[low_port].as_ref().unwrap();
    let high_port_frames = game.frames[high_port].as_ref().unwrap();
    let low_starting_character = game.info.starting_character_colours[low_port].unwrap();
    let high_starting_character = game.info.starting_character_colours[high_port].unwrap();

    // search backwards for a good frame to export -------------------------

    // max number of frames to search back
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

        true
    }

    while !good_frame(&low_port_frames[frame]) || !good_frame(&high_port_frames[frame]) {
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

    if frame + duration >= low_port_frames.len() { return Err(ReplayCreationError::RecordingOutOfBounds) }
    if name.len() >= 32 { return Err(ReplayCreationError::FilenameTooLong) }
    if !name.is_ascii() { return Err(ReplayCreationError::FilenameNotASCII) }
    if duration > 3600 { return Err(ReplayCreationError::DurationTooLong) }

    let info = &game.info;
    let time = info.start_time.fields();

    let mut filename = [0u8; 31];
    filename[..name.len()].copy_from_slice(name.as_bytes());

    fn inputs_over_frames(frames: &[slp_parser::Frame]) -> Vec<Input> {
        use slp_parser::buttons_mask;

        frames
            .iter()
            .map(|f| {
                Input {
                    button_flags: (
                          ((f.buttons_mask & buttons_mask::Z) >> 4)
                            | ((f.buttons_mask & buttons_mask::R_DIGITAL) >> 4)
                            | ((f.buttons_mask & buttons_mask::L_DIGITAL) >> 4)
                            | ((f.buttons_mask & buttons_mask::X) >> 7)
                            | ((f.buttons_mask & buttons_mask::Y) >> 7)
                            | ((f.buttons_mask & buttons_mask::B) >> 4)
                            | ((f.buttons_mask & buttons_mask::A) >> 2)
                            | ((f.buttons_mask & buttons_mask::D_PAD_UP) << 4)
                    ) as u8,
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
        match prev_frame {
            Some(p) => {
                prev_position = [p.position.x, p.position.y, 0.0];
                prev_stick = vector_to_arr(p.left_stick_coords);
            }
            None => {
                prev_position = [frame.position.x, frame.position.y, 0.0];
                prev_stick = [0.0, 0.0];
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

        let mut char_state_var = [0u8; 72];
        char_state_var[0..4].copy_from_slice(&frame.hitstun_misc.to_be_bytes());

        // only first char state var is recorded. The rest we have to calculate, if they matter.
        match frame.state {
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::Turn) => {
                let dir = match frame.direction {
                    slp_parser::Direction::Left => 1.0f32,
                    slp_parser::Direction::Right => -1.0f32,
                };
                char_state_var[4..8].copy_from_slice(&dir.to_be_bytes());
                char_state_var[8..12].copy_from_slice(&(-dir).to_be_bytes());
            }
            slp_parser::ActionState::Standard(slp_parser::StandardActionState::CliffWait) => {
                // prevents immediate fall from ledge
                char_state_var[4..8].copy_from_slice(&640.0f32.to_be_bytes());

                // prevents other character from grabbing ledge
                char_state_var[8..12].copy_from_slice(&1u32.to_be_bytes());
                char_state_var[12..16].copy_from_slice(&40.0f32.to_be_bytes());
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

        let subaction_flags = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let mut stale_count = 0;
        let mut stale_moves = [StaleMove::NULL; 10];

        let mut i = frame_idx;
        let mut prev_instance_id = u16::MAX;
        loop {
            let instance_id = opponent_frames[i].last_hit_by_instance_id;

            if instance_id != prev_instance_id {
                let move_id = frames[i].last_hitting_attack_id;
                if move_id == 0 { break; } // end on death

                // id 1 does not stale
                if move_id != 1 { 
                    stale_moves[stale_count] = StaleMove { move_id, instance_id };
                    stale_count += 1;
                    if stale_count == 10 { break; }
                    prev_instance_id = instance_id;
                }
            }

            if i == 0 { break }
            i -= 1;
        }

        // reverse order, since we iterated backwards
        stale_moves[..stale_count].reverse();

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
            char_state_var,
            hitlag_frames_left: frame.hitlag_frames,
            subaction_flags,
            state_flags: frame.state_flags,
            stale_moves,

            prev_position,
            prev_stick,
            stick: vector_to_arr(frame.left_stick_coords),
            cstick: vector_to_arr(frame.right_stick_coords),
            trigger: frame.analog_trigger_value,

            // state_blend, x_rotn_rot, anim_velocity
            ..Default::default()
        }
    }

    let (hmn_char, hmn_frames, cpu_char, cpu_frames) = match human {
        HumanPort::HumanLowPort => (
            low_starting_character, &low_port_frames,
            high_starting_character, &high_port_frames,
        ),
        HumanPort::HumanHighPort => (
            high_starting_character, &high_port_frames,
            low_starting_character, &low_port_frames,
        ),
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
            hmn_state: state(hmn_char, hmn_frames, cpu_frames, frame),
            cpu_state: state(cpu_char, cpu_frames, hmn_frames, frame),
        },
        &InputRecordings {
            hmn_slots: [
                Some(&inputs_over_frames(&hmn_frames[frame+1..frame+duration+1])),
                None, None, None, None, None
            ],
            cpu_slots: [
                Some(&inputs_over_frames(&cpu_frames[frame+1..frame+duration+1])),
                None, None, None, None, None
            ],
        }
    )
}
