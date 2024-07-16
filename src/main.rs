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

        let seconds = SystemTime:now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let days = seconds / (60 * 60 * 24);

        let mut year = 1970;
        let mut days_in_year;
        while (1) {
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
            if days_in_year == 365 && days >= 29 { days -= 29 } else { break 2 }
            if days_in_year == 366 && days >= 30 { days -= 30 } else { break 2 }
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
        }

        days += 1; // one-index the day

        RecordingTime {
            year,
            month,
            day,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RecordingInfo {
    // header items
    pub hmn     : slp_parser::CharacterColour,
    pub cpu     : slp_parser::CharacterColour,
    pub stage   : slp_parser::Stage,
    pub time    : RecordingTime,
    pub filename: [u8; 32], // ascii

    pub menu_settings: RecordingMenuSettings,
}

impl RecordingInfo {
    // offsets zeroed but not written
    fn write_header(&self, b: &mut Vec<u8>) {
        let char_hmn = self.hmn.character().to_u8_external().unwrap();
        let costume_hmn = self.hmn.costume_idx();
        let char_cpu = self.cpu.character().to_u8_external().unwrap();
        let costume_cpu = self.cpu.costume_idx();

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
        b.extend_from_slice(&[0u8; 3]); // 3 bytes padding
        b.extend_from_slice(&[0u8; 12]); // 3 offsets
    }
    
    fn write_menu_settings(&self, b: &mut Vec<u8>) {
        b.extend_from_slice(&[
            menu.hmn_mode as u8,
            menu.hmn_slot as u8,
            menu.cpu_mode as u8,
            menu.cpu_slot as u8,
            menu.loop_inputs as u8,
            menu.auto_restore as u8,
        ]);
    }
}

#[repr(C)]
pub struct RecInputData {
    pub start_frame: u32, // the frame these inputs start on
    pub num: u32,
    pub inputs: [RecInputs; REC_LENGTH],
}

#[derive(Clone, Debug)]
pub struct CharacterState {
    pub position: [f32; 2],
    pub state: slp_parser::ActionState,
    //pub stale_queue: [i32; 11],
    //pub air_velocity: [f32; 2],
    //pub hit_velocity: [f32; 2],
    pub percent: f32,
},

#[derive(Copy, Clone, Debug)]
pub struct InitialState {
    pub hmn: CharacterState,
    pub cpu: CharacterState,
}

pub struct RecInputs {
    // btn_dpadup : 1;
    // btn_a : 1;
    // btn_b : 1;
    // btn_x : 1;
    // btn_y : 1;
    // btn_L : 1;
    // btn_R : 1;
    // btn_Z : 1;
    pub button_flags: u8,
    pub stickX: i8,
    pub stickY: i8,
    pub substickX: i8,
    pub substickY: i8,
    pub trigger: u8,
}

pub struct InputRecordings<'a> {
    pub hmn: &'a [RecInputs],
    pub cpu: &'a [RecInputs],
}

const EVENT_DATASIZE: usize = 512;
const REC_LENGTH: usize = 1 * 60 * 60; // 60 seconds
const REC_SLOTS: usize = 6;

static DEFAULT_SAVESTATE_AND_MATCHINIT: &'static [u8] = include_bytes!("../savestate_matchinit.raw");
const SAVESTATE_SIZE: usize = 54616;
const MATCHINIT_SIZE: usize = 312;
const RECORDING_SIZE: usize = 314224;

static DEFAULT_GCI_HEADER: &'static [u8] = include_bytes!("../gci_header.raw");

const BLOCK_SIZE: usize = 8192;

// puts in big endian format
pub fn construct_tm_replay_buffer(
    info: &RecordingInfo, 
    //initial_state: &InitialState,
    //recording: &InputRecordings,
) -> (Vec<u8>, String) {
    let mut bytes = Vec::with_capacity(8192 * 8);

    info.write_header(&mut bytes);

    let screenshot_offset = bytes.len();
    let screenshot_size = 2 * 96 * 72;
    bytes.resize(68 + screenshot_size, 0u8); // black screen for now

    let recording_offset = bytes.len();

    let mut recording_save = vec![0u8; RECORDING_SIZE + 257]; // pad a bit for compression
    let rec_start = SAVESTATE_SIZE+MATCHINIT_SIZE;
    recording_save[0..rec_start].copy_from_slice(&DEFAULT_SAVESTATE_AND_MATCHINIT[..rec_start]);

    let rec_slot_size = 4 + 4 + REC_SLOTS*REC_LENGTH;
    // hmn
    for i in 0..REC_SLOTS {
        let input_data_start = rec_start + i*rec_slot_size;

        recording_save[input_data_start+0..input_data_start+4].copy_from_slice(&(-1i32).to_be_bytes()); // start_frame
        recording_save[input_data_start+4..input_data_start+8].copy_from_slice(&0u32.to_be_bytes());    // num_frames
        // zero buttons
    }

    // cpu
    for i in 0..REC_SLOTS {
        let input_data_start = rec_start + (i+6)*rec_slot_size;

        recording_save[input_data_start+0..input_data_start+4].copy_from_slice(&(-1i32).to_be_bytes()); // start_frame
        recording_save[input_data_start+4..input_data_start+8].copy_from_slice(&0u32.to_be_bytes());    // num_frames
        // zero buttons
    }

    // make space for compression
    bytes.resize(recording_offset + RECORDING_SIZE, 0u8);
    let recording_compressed_size = compress::lz77_compress(
        &recording_save, 
        RECORDING_SIZE as u32, 
        &mut bytes[recording_offset..]
    ) as usize;
    bytes.resize(recording_offset+recording_compressed_size, 0u8);

    let menu_settings_offset = bytes.len();

    info.write_menu_settings(&mut bytes);

    bytes[56..60].copy_from_slice(&(screenshot_offset as u32).to_be_bytes());
    bytes[60..64].copy_from_slice(&(recording_offset as u32).to_be_bytes());
    bytes[64..68].copy_from_slice(&(menu_settings_offset as u32).to_be_bytes());

    let filename = format!(
        "01-GTME-TMREC_{:02}{:02}{:04}_{:02}{:02}{:02}.gci", 
        header.month, header.day, header.year,
        header.hour, header.minute, header.second,
    );

    (bytes, filename)
}

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

fn encode_block(data: &mut [u8], len: usize) {
    let (checksum, encoded_data) = data.split_at_mut(16);
    calculate_checksum(encoded_data, checksum);

    for i in 16..len {
        data[i] = obfuscate_byte(data[i-1], data[i]);
    }
}

const WEIRD_BLOCK_HEADER: [u8; 16] = [0, 0, 0, 0, 0xc0, 0x4d, 0x20, 0x01, 0x00, 0x00, 0x00, 0x02, 0, 0, 0, 0];

pub fn construct_tm_replay_gci(replay_buffer: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(8096 * 8);

    bytes.extend_from_slice(DEFAULT_GCI_HEADER);

    let name = b"custom tmrec";
    bytes[0x60..0x60+name.len()].copy_from_slice(name);

    assert!(bytes.len() == 0x1EB0);

    // round up division by block size
    // subtract 400 bytes, because first block always has that size for some reason
    let full_blocks = (replay_buffer.len() - (400-32) + (BLOCK_SIZE-32) - 1) / (BLOCK_SIZE-32);
    
    bytes.resize(bytes.len() + 16, 0); // space for checksum
    bytes.extend_from_slice(&WEIRD_BLOCK_HEADER);
    bytes.extend_from_slice(&replay_buffer[0..(400 - 32)]);

    for i in 0..full_blocks {
        bytes.resize(bytes.len() + 16, 0); // space for checksum
        bytes.extend_from_slice(&WEIRD_BLOCK_HEADER);
        let len = bytes.len();
        bytes[len - 16 + 3] = i as u8; // block idx

        let block_data_start = (400-32) + (BLOCK_SIZE-32)*i;
        if replay_buffer[block_data_start..].len() >= (BLOCK_SIZE-32) {
            bytes.extend_from_slice(&replay_buffer[block_data_start..block_data_start+BLOCK_SIZE-32]);
        } else {
            bytes.extend_from_slice(&replay_buffer[block_data_start..]);
        }
    }

    // fill out last block
    bytes.resize(0x1EB0 + 400 + BLOCK_SIZE*full_blocks, 0u8);

    encode_block(&mut bytes[0x1EB0..0x1EB0+400], 400);

    for i in 0..full_blocks {
        let start = 0x1EB0 + 400 + BLOCK_SIZE*i;
        encode_block(&mut bytes[start..start+BLOCK_SIZE], BLOCK_SIZE);
    }

    bytes
}

fn main() {
    let mut filename_buf = [0u8; 32];
    let name = b"Hello, World!";
    filename_buf[0..name.len()].copy_from_slice(name);

    let (replay, filename) = construct_tm_replay_buffer(
        &ReplayInfo {
            hmn: slp_parser::Character::Fox.neutral(),
            cpu: slp_parser::Character::Falco.neutral(),
            stage: slp_parser::Stage::Battlefield,
            time: RecordingTime::today_approx(),
            filename: filename_buf,
            menu_settings: RecordingMenuSettings::default(),
        },
    );

    let gci = construct_tm_replay_gci(&replay);

    let mut path = std::path::PathBuf::from("/home/alex/.config/SlippiOnline/GC/USA/Card A/");
    path.push(&filename);
    std::fs::write(&path, &gci).unwrap();
    println!("wrote replay {}", path.display());
}
