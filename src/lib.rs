mod compress;

// Things that could stale but don't
// - luigi's taunt
// - zair
// - dk cargo throw release
#[derive(Copy, Clone, Debug)]
pub enum StaleableMoves {
    Jab1 = 0x02,
    Jab2 = 0x03,
    Jab3 = 0x04,
    JabRapid = 0x05,
    DashAttack = 0x06,
    FTilt = 0x07,
    UTilt = 0x08,
    DTilt = 0x09,
    FSmash = 0x0A,
    USmash = 0x0B,
    DSmash = 0x0C,

    NAir = 0x0D,
    FAir = 0x0E,
    BAir = 0x0F,
    UAir = 0x10,
    DAir = 0x11,

    NSpecial = 0x12,
    SSpecial = 0x13,
    USpecial = 0x14,
    DSpecial = 0x15,

    // Kirby copy abilities
    NSpecialCopyMario          = 0x16,
    NSpecialCopyFox            = 0x17,
    NSpecialCopyCaptainFalcon  = 0x18,
    NSpecialCopyDonkeyKong     = 0x19,
    NSpecialCopyBowser         = 0x1A,
    NSpecialCopyLink           = 0x1B,
    NSpecialCopySheik          = 0x1C,
    NSpecialCopyNess           = 0x1D,
    NSpecialCopyPeach          = 0x1E,
    NSpecialCopyIceClimbers    = 0x1F,
    NSpecialCopyPikachu        = 0x20,
    NSpecialCopySamus          = 0x21,
    NSpecialCopyYoshi          = 0x22,
    NSpecialCopyJigglypuff     = 0x23,
    NSpecialCopyMewtwo         = 0x24,
    NSpecialCopyLuigi          = 0x25,
    NSpecialCopyMarth          = 0x26,
    NSpecialCopyZelda          = 0x27,
    NSpecialCopyYoungLink      = 0x28,
    NSpecialCopyDrMario        = 0x29,
    NSpecialCopyFalco          = 0x2A,
    NSpecialCopyPichu          = 0x2B,
    NSpecialCopyMrGameAndWatch = 0x2C,
    NSpecialCopyGanondorf      = 0x2D,
    NSpecialCopyRoy            = 0x2E,

    GetUpAttackBack = 0x32,
    GetUpAttackStomach = 0x33,

    Pummel = 0x34,
    FThrow = 0x35,
    BThrow = 0x36,
    UThrow = 0x37,
    DThrow = 0x38,

    // DK cargo throws
    FCargoThrow = 0x39,
    BCargoThrow = 0x3A,
    UCargoThrow = 0x3B,
    DCargoThrow = 0x3C,

    LedgeAttackSlow = 0x3D,
    LedgeAttackFast = 0x3E,
}

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
pub struct RecordingState<'a> {
    pub time: RecordingTime,
    /// Name to show when browsing in Training Mode.
    pub filename: [u8; 32], // ascii
    pub menu_settings: RecordingMenuSettings,

    /// Melee starts at frame -123. 'GO' disappears on frame 0.
    pub start_frame: i32,
    pub stage: slp_parser::Stage,
    pub hmn_state: CharacterState<'a>,
    pub cpu_state: CharacterState<'a>,
}

impl RecordingState<'_> {
    // offsets zeroed but not written
    fn write_header(&self, b: &mut Vec<u8>) {
        let char_hmn = self.hmn_state.character.character().to_u8_external().unwrap();
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
pub struct CharacterState<'a> {
    pub character: slp_parser::CharacterColour,
    pub position: [f32; 2],
    pub airborne: bool,
    pub state: slp_parser::ActionState,
    pub state_frame: f32,
    pub direction: slp_parser::Direction,
    pub percent: f32,

    /// Truncated to the first ten items.
    pub stale_moves: &'a [StaleableMoves],
    pub anim_velocity: [f32; 2],
    pub self_velocity: [f32; 2],
    pub hit_velocity: [f32; 2],
    pub ground_velocity: [f32; 2],

    pub prev_position: [f32; 2],
    pub stick: [f32; 2],
    pub cstick: [f32; 2],
    pub prev_stick: [f32; 2],
    pub trigger: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct RecInputs {
    /// - z: 0x01
    /// - r digital: 0x02
    /// - l digital: 0x04
    /// - x: 0x08
    /// - y: 0x10
    /// - b: 0x20
    /// - a: 0x40
    /// - dpad up: 0x80
    pub button_flags: u8,

    pub stick_x: i8,
    pub stick_y: i8,
    pub cstick_x: i8,
    pub cstick_y: i8,
    pub trigger: u8,
}

impl RecInputs {
    pub const NO_INPUT: RecInputs = RecInputs {
        button_flags: 0, stick_x: 0, stick_y: 0, cstick_x: 0, cstick_y: 0, trigger: 0,
    };
}

#[derive(Copy, Clone, Debug)]
pub struct InputRecordings<'a> {
    /// Each slot is truncated to 3600 frames.
    pub hmn_slots: [Option<&'a [RecInputs]>; 6],

    /// Each slot is truncated to 3600 frames.
    pub cpu_slots: [Option<&'a [RecInputs]>; 6],
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

// 380*0x20 bytes taken from 0x803C2800
static ACTION_FN_LOOKUP_TABLE: &'static [u8] = include_bytes!("fn_table.raw");

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

// Overwrites the RecordingSave in a replay buffer. You probably don't want this.
//
// Will resize recording_save.
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
    filename: &[u8; 32],
    replay_buffer: &[u8],
) -> Vec<u8> {
    // for the gci file
    let mut bytes = Vec::with_capacity(8096 * 8);

    bytes.extend_from_slice(DEFAULT_GCI_HEADER);

    let ident = "GTME01";
    bytes[0..6].copy_from_slice(ident.as_bytes());
    let gci_inner_name = format!(
        "TMREC_{:02}{:02}{:04}_{:02}{:02}{:02}", 
        date.month, date.day, date.year,
        date.hour, date.minute, date.second,
    );
    bytes[8..0x28].fill(0);
    bytes[8..8+gci_inner_name.len()].copy_from_slice(gci_inner_name.as_bytes());

    bytes[0x60..][..32].copy_from_slice(filename);

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

    bytes
}

/// Construct TM replay from initial state and inputs.
///
/// Returns GCI file bytes.
pub fn construct_tm_replay(
    state: &RecordingState, 
    inputs: &InputRecordings,
) -> Vec<u8> {
    // buffer created by unclepunch's tm code
    let mut bytes = Vec::with_capacity(8192 * 8);

    state.write_header(&mut bytes);

    let screenshot_offset = bytes.len();
    let screenshot_size = 2 * 96 * 72;
    bytes.resize(68 + screenshot_size, 0u8); // black screen for now

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
    
    fn write_ft_state_data(ft_state: &mut [u8], st: &CharacterState) {
        // nested struct offsets
        let phys_offset = 40;
        let input_offset = 568;
        let dmg_offset = 3680;

        ft_state[input_offset..][..4].copy_from_slice(&st.stick[0].to_be_bytes());
        ft_state[input_offset+4..][..4].copy_from_slice(&st.stick[1].to_be_bytes());
        ft_state[input_offset+8..][..4].copy_from_slice(&st.prev_stick[0].to_be_bytes());
        ft_state[input_offset+12..][..4].copy_from_slice(&st.prev_stick[1].to_be_bytes());
        ft_state[input_offset+24..][..4].copy_from_slice(&st.cstick[0].to_be_bytes());
        ft_state[input_offset+28..][..4].copy_from_slice(&st.cstick[1].to_be_bytes());
        ft_state[input_offset+48..][..4].copy_from_slice(&st.trigger.to_be_bytes());

        // phys struct
        let state_offset = 4;
        ft_state[state_offset..][..4].copy_from_slice(&(st.state.as_u16() as u32).to_be_bytes());
        let direction_bytes = match st.direction {
            slp_parser::Direction::Left => (-1.0f32).to_be_bytes(),
            slp_parser::Direction::Right => 1.0f32.to_be_bytes(),
        };
        ft_state[state_offset+4..][..4].copy_from_slice(&direction_bytes);
        ft_state[state_offset+8..][..4].copy_from_slice(&st.state_frame.to_be_bytes());
        ft_state[state_offset+12..][..4].copy_from_slice(&(1.0f32).to_be_bytes()); // state speed
        ft_state[state_offset+16..][..4].copy_from_slice(&(0.0f32).to_be_bytes()); // state blend
        
        let vel_offset = phys_offset;
        ft_state[vel_offset+0..][..4].copy_from_slice(&st.anim_velocity[0].to_be_bytes()); // anim_vel.x
        ft_state[vel_offset+4..][..4].copy_from_slice(&st.anim_velocity[1].to_be_bytes()); // anim_vel.y
        ft_state[vel_offset+12+0..][..4].copy_from_slice(&st.self_velocity[0].to_be_bytes()); // self_vel.x
        ft_state[vel_offset+12+4..][..4].copy_from_slice(&st.self_velocity[1].to_be_bytes()); // self_vel.y
        ft_state[vel_offset+24+0..][..4].copy_from_slice(&st.hit_velocity[0].to_be_bytes()); // kb_vel.x
        ft_state[vel_offset+24+4..][..4].copy_from_slice(&st.hit_velocity[1].to_be_bytes()); // kb_vel.y
        ft_state[vel_offset+120+0..][..4].copy_from_slice(&st.ground_velocity[0].to_be_bytes()); // selfVelGround.x
        ft_state[vel_offset+120+4..][..4].copy_from_slice(&st.ground_velocity[1].to_be_bytes()); // selfVelGround.y
        
        let pos_offset = phys_offset + 12*3 + 4*6;
        let x_pos_bytes = st.position[0].to_be_bytes();
        let y_pos_bytes = st.position[1].to_be_bytes();
        ft_state[pos_offset+0..][..4].copy_from_slice(&x_pos_bytes); // pos.x
        ft_state[pos_offset+4..][..4].copy_from_slice(&y_pos_bytes); // pos.y
        //ft_state[pos_offset+8..][..4].copy_from_slice(&(10.0f32).to_be_bytes());  // pos_delta.x
        //ft_state[pos_offset+12..][..4].copy_from_slice(&(0.0f32).to_be_bytes()); // pos_delta.y
        //ft_state[pos_offset+16..][..4].copy_from_slice(&x_pos_bytes); // pos_prev.x
        //ft_state[pos_offset+20..][..4].copy_from_slice(&y_pos_bytes); // pos_prev.y

        let airborne_offset = phys_offset + 108; // air_state in struct phys
        ft_state[airborne_offset..][..4].copy_from_slice(&(st.airborne as u32).to_be_bytes());

        let percent_offset = dmg_offset + 4;
        let percent_bytes = (st.percent*0.5).to_be_bytes(); // percent is stored halved for some reason???
        ft_state[percent_offset..][..4].copy_from_slice(&percent_bytes);
        ft_state[percent_offset+8..][..4].copy_from_slice(&percent_bytes); // temp percent???
        
        // action state functions
        let fns_offset = (st.state.as_u16() as usize) * 0x20;
        let fns = &ACTION_FN_LOOKUP_TABLE[fns_offset+0xC..fns_offset+0x20]; // 5 fn pointers
        ft_state[0x10CC..][..4].copy_from_slice(&fns[4..8]); // IASA
        ft_state[0x10CC..][4..8].copy_from_slice(&fns[0..4]); // Anim
        ft_state[0x10CC..][8..20].copy_from_slice(&fns[8..20]); // Phys, Coll, Cam

        // stale moves

        let stale_moves_trunc = &st.stale_moves[0..st.stale_moves.len().min(10)];

        let stale_offset = 8972;
        let next_stale_move_idx = (stale_moves_trunc.len() as u32) % 10;
        ft_state[stale_offset..][..4].copy_from_slice(&next_stale_move_idx.to_be_bytes());

        for i in 0..stale_moves_trunc.len() {
            let offset = stale_offset + 4 + 4*i;
            let move_id = stale_moves_trunc[i] as u16;
            ft_state[offset..][..2].copy_from_slice(&move_id.to_be_bytes());
            ft_state[offset+2..][..2].copy_from_slice(&[0, 0]); // # of action states this game (unused probably)
        }
       
        for i in stale_moves_trunc.len()..10 {
            let offset = stale_offset + 4 + 4*i;
            ft_state[offset..][..4].copy_from_slice(&[0, 0, 0, 0]);
        }
    }

    let st_offset = 312; // savestate offset - skip MatchInit in RecordingSave
    let ft_state_offset = 8+EVENT_DATASIZE; // FtState array offset - fields in Savestate;
    let ft_state_size = 9016;
    write_ft_state_data(&mut recording_save[st_offset+ft_state_offset..][..ft_state_size], &state.hmn_state);
    write_ft_state_data(&mut recording_save[st_offset+ft_state_offset+ft_state_size..][..ft_state_size], &state.cpu_state);

    // write inputs

    recording_save[rec_start+0..rec_start+4].copy_from_slice(&0u32.to_be_bytes()); // start_frame
    recording_save[rec_start+4..rec_start+8].copy_from_slice(&60u32.to_be_bytes()); // num_frames
    for i in 0..60 {
        let o = rec_start+8+6*i;
        recording_save[o..o+6].copy_from_slice(&[
            0u8, // buttons
            127i8 as u8,
            0,
            0,
            0,
            0,
        ]); // inputs
    }

    fn write_inputs(slot: &mut [u8], start_frame: i32, inputs: Option<&[RecInputs]>) {
        // if None or len == 0
        if !inputs.is_some_and(|i| !i.is_empty()) {
            slot[0..4].copy_from_slice(&(-1i32).to_be_bytes()); // start_frame
            slot[4..8].copy_from_slice(&0u32.to_be_bytes());    // num_frames
            return;
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
    }

    // hmn inputs
    for i in 0..REC_SLOTS {
        let input_data_start = rec_start + i*REC_SLOT_SIZE;
        let slot = &mut recording_save[input_data_start..][..REC_SLOT_SIZE];
        write_inputs(slot, state.start_frame, inputs.hmn_slots[i]);
    }

    // cpu inputs
    for i in 0..REC_SLOTS {
        let input_data_start = rec_start + (i+6)*REC_SLOT_SIZE;
        let slot = &mut recording_save[input_data_start..][..REC_SLOT_SIZE];
        write_inputs(slot, state.start_frame, inputs.cpu_slots[i]);
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

/// Construct TM replay from slp file.
///
/// Returns GCI file bytes.
///
/// # Unimplemented
/// - stale moves
/// - items
/// - animation speed
/// - animation blending
///
/// # Panics
/// - If frame + duration is out of bounds.
/// - If duration is greater than 3600 frames
/// - If name is longer than 31 bytes
/// - If name is not ASCII
pub fn construct_tm_replay_from_slp(
    game: &slp_parser::Game, 
    frame: usize,
    duration: usize,
    name: &str,
) -> Vec<u8> {
    assert!(frame + duration < game.low_port_frames.len());
    assert!(name.len() < 32);
    assert!(name.is_ascii());
    assert!(duration <= 3600);

    let info = &game.info;
    let time = info.start_time.fields();

    let mut filename = [0u8; 32];
    filename[..name.len()].copy_from_slice(name.as_bytes());

    fn inputs_over_frames(frames: &[slp_parser::Frame]) -> Vec<RecInputs> {
        use slp_parser::buttons_mask;

        frames
            .iter()
            .map(|f| {
                RecInputs {
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
                    stick_x: (f.left_stick_coords[0] * 80.0) as i8,
                    stick_y: (f.left_stick_coords[1] * 80.0) as i8,
                    cstick_x: (f.right_stick_coords[0] * 80.0) as i8,
                    cstick_y: (f.right_stick_coords[1] * 80.0) as i8,
                    trigger: (f.analog_trigger_value * 140.0) as u8,
                }
            }).collect()
    }

    fn state(
        starting_char: slp_parser::CharacterColour, 
        prev_frame: Option<&slp_parser::Frame>,
        frame: &slp_parser::Frame,
    ) -> CharacterState<'static> {
    
        let prev_position;
        let prev_stick;
        match prev_frame {
            Some(p) => {
                prev_position = [p.position.x, p.position.y];
                prev_stick = p.left_stick_coords;
            }
            None => {
                prev_position = [frame.position.x, frame.position.y];
                prev_stick = [0.0, 0.0];
            }
        }

        CharacterState {
            // respect zelda/sheik transformation
            character: slp_parser::CharacterColour::from_character_and_colour(
               frame.character, 
               starting_char.costume_idx()
            ).unwrap(),
            position: [frame.position.x, frame.position.y],
            airborne: frame.is_airborne,
            state: frame.state,
            state_frame: frame.anim_frame,
            direction: frame.direction,
            percent: frame.percent,
            stale_moves: &[],
            anim_velocity: [0.0; 2], // IDK
            self_velocity: [frame.velocity.x, frame.velocity.y],
            hit_velocity: [frame.hit_velocity.x, frame.hit_velocity.y],
            ground_velocity: [frame.ground_x_velocity, 0.0],

            prev_position,
            prev_stick,
            stick: frame.left_stick_coords,
            cstick: frame.right_stick_coords,
            trigger: frame.analog_trigger_value,
        }
    }

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
            hmn_state: state(info.low_starting_character, game.low_port_frames.get(frame-1), &game.low_port_frames[frame]),
            cpu_state: state(info.high_starting_character, game.low_port_frames.get(frame-1), &game.high_port_frames[frame]),
        },
        &InputRecordings {
            hmn_slots: [
                Some(&inputs_over_frames(&game.low_port_frames[frame+1..frame+duration+1])),
                None, None, None, None, None
            ],
            cpu_slots: [
                Some(&inputs_over_frames(&game.high_port_frames[frame+1..frame+duration+1])),
                None, None, None, None, None
            ],
        }
    )
}
