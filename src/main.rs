mod compress;

use arrayvec::ArrayVec;

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

#[derive(Copy, Clone, Debug)]
pub enum Direction { Left, Right }

#[derive(Clone, Debug)]
pub struct CharacterState {
    pub position: [f32; 2],
    pub airborne: bool,
    pub state: slp_parser::ActionState,
    pub state_frame: f32,
    pub direction: Direction,
    pub percent: f32,
    pub stale_moves: ArrayVec<StaleableMoves, 10>,
    pub anim_velocity: [f32; 2],
    pub self_velocity: [f32; 2],
    pub hit_velocity: [f32; 2],
    pub ground_velocity: [f32; 2],
}

#[derive(Clone, Debug)]
pub struct InitialState {
    pub start_frame: i32,
    pub hmn: CharacterState,
    pub cpu: CharacterState,
}

#[derive(Copy, Clone, Debug)]
pub struct RecInputs {
    // btn_dpadup : 1;
    // btn_a : 1;
    // btn_b : 1;
    // btn_x : 1;
    // btn_y : 1;
    // btn_l : 1;
    // btn_r : 1;
    // btn_z : 1;
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

pub struct InputRecordings<'a> {
    pub hmn: &'a [&'a [RecInputs]],
    pub cpu: &'a [&'a [RecInputs]],
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

fn encode_block(data: &mut [u8], len: usize) {
    let (checksum, encoded_data) = data.split_at_mut(16);
    calculate_checksum(encoded_data, checksum);

    for i in 16..len {
        data[i] = obfuscate_byte(data[i-1], data[i]);
    }
}

const WEIRD_BLOCK_HEADER: [u8; 16] = [0, 0, 0, 0, 0xc0, 0x4d, 0x1d, 0x01, 0x00, 0x00, 0x00, 0x02, 0, 0, 0, 0];

// inputs will be truncated if they are longer than 3600 frames.
// There is a a maximum of 6 slots each for both human and cpu.
pub fn construct_tm_replay(
    info: &RecordingInfo, 
    initial_state: &InitialState,
    inputs: &InputRecordings,
) -> (Vec<u8>, String) {
    // buffer created by unclepunch's tm code
    let replay_buffer = {
        let mut bytes = Vec::with_capacity(8192 * 8);

        info.write_header(&mut bytes);

        let screenshot_offset = bytes.len();
        let screenshot_size = 2 * 96 * 72;
        bytes.resize(68 + screenshot_size, 0u8); // black screen for now

        let recording_offset = bytes.len();

        let mut recording_save = vec![0u8; RECORDING_SIZE + 257]; // pad a bit for compression algo
        let rec_start = SAVESTATE_SIZE+MATCHINIT_SIZE;
        recording_save[0..rec_start].copy_from_slice(&DEFAULT_SAVESTATE_AND_MATCHINIT[..rec_start]);

        let savestate_offset = MATCHINIT_SIZE;
        recording_save[savestate_offset+4..][..4].copy_from_slice(&initial_state.start_frame.to_be_bytes());

        // overwrite RecordingSave values

        fn write_ft_state_data(ft_state: &mut [u8], st: &CharacterState) {
            // nested struct offsets
            let phys_offset = 40;
            let dmg_offset = 3680;

            let state_offset = 4;
            ft_state[state_offset..][..4].copy_from_slice(&(st.state.as_u16() as u32).to_be_bytes());
            ft_state[state_offset+4..][..4].copy_from_slice(&st.state_frame.to_be_bytes());

            let direction_offset = 8;
            let direction_bytes = match st.direction {
                Direction::Left => (-1.0f32).to_be_bytes(),
                Direction::Right => 1.0f32.to_be_bytes(),
            };
            ft_state[direction_offset..][..4].copy_from_slice(&direction_bytes);
            
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
            ft_state[pos_offset+12..][..4].copy_from_slice(&x_pos_bytes); // pos_prev.x
            ft_state[pos_offset+12+4..][..4].copy_from_slice(&y_pos_bytes); // pos_prev.y

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

            let stale_offset = 8972;
            let next_stale_move_idx = (st.stale_moves.len() as u32) % 10;
            ft_state[stale_offset..][..4].copy_from_slice(&next_stale_move_idx.to_be_bytes());

            for i in 0..st.stale_moves.len() {
                let offset = stale_offset + 4 + 4*i;
                let move_id = st.stale_moves[i] as u16;
                ft_state[offset..][..2].copy_from_slice(&move_id.to_be_bytes());
                ft_state[offset+2..][..2].copy_from_slice(&[0, 0]); // # of action states this game (unused probably)
            }
           
            for i in st.stale_moves.len()..10 {
                let offset = stale_offset + 4 + 4*i;
                ft_state[offset..][..4].copy_from_slice(&[0, 0, 0, 0]);
            }
        }

        let st_offset = 312; // savestate offset - skip MatchInit in RecordingSave
        let ft_state_offset = 8+EVENT_DATASIZE; // FtState array offset - fields in Savestate;
        let ft_state_size = 9016;
        write_ft_state_data(&mut recording_save[st_offset+ft_state_offset..][..ft_state_size], &initial_state.hmn);
        write_ft_state_data(&mut recording_save[st_offset+ft_state_offset+ft_state_size..][..ft_state_size], &initial_state.cpu);

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

        fn write_inputs(slot: &mut [u8], start_frame: i32, inputs: &[RecInputs]) {
            if inputs.len() == 0 {
                slot[0..4].copy_from_slice(&(-1i32).to_be_bytes()); // start_frame
                slot[4..8].copy_from_slice(&0u32.to_be_bytes());    // num_frames
                return;
            }

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
        for i in 0..inputs.hmn.len().min(REC_SLOTS) {
            let input_data_start = rec_start + i*REC_SLOT_SIZE;
            let slot = &mut recording_save[input_data_start..][..REC_SLOT_SIZE];
            write_inputs(slot, initial_state.start_frame, inputs.hmn[i]);
        }

        // cpu inputs
        for i in 0..inputs.cpu.len().min(REC_SLOTS) {
            let input_data_start = rec_start + (i+6)*REC_SLOT_SIZE;
            let slot = &mut recording_save[input_data_start..][..REC_SLOT_SIZE];
            write_inputs(slot, initial_state.start_frame, inputs.cpu[i]);
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

        info.write_menu_settings(&mut bytes);

        bytes[56..60].copy_from_slice(&(screenshot_offset as u32).to_be_bytes());
        bytes[60..64].copy_from_slice(&(recording_offset as u32).to_be_bytes());
        bytes[64..68].copy_from_slice(&(menu_settings_offset as u32).to_be_bytes());

        bytes
    };

    {
        let path = std::path::PathBuf::from("/home/alex/melee/tutor/tm_replay_parser/blank2.raw");
        std::fs::write(&path, &replay_buffer).unwrap();
        println!("wrote raw replay {}", path.display());
    }

    // for the gci file
    let mut bytes = Vec::with_capacity(8096 * 8);

    bytes.extend_from_slice(DEFAULT_GCI_HEADER);

    let date = info.time;
    let ident = "GTME01";
    bytes[0..6].copy_from_slice(ident.as_bytes());
    let gci_inner_name = format!(
        "TMREC_{:02}{:02}{:04}_{:02}{:02}{:02}", 
        date.month, date.day, date.year,
        date.hour, date.minute, date.second,
    );
    bytes[8..0x28].fill(0);
    bytes[8..8+gci_inner_name.len()].copy_from_slice(gci_inner_name.as_bytes());

    bytes[0x60..0x60+info.filename.len()].copy_from_slice(&info.filename);

    assert!(bytes.len() == 0x1EB0);

    // recalculate gci header checksum

    let (gci_header, checksum) = bytes.split_at_mut(0x1E80);
    calculate_checksum(&gci_header[0x40..], checksum);

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
        bytes[len - 16 + 1] = i as u8 + 1; // block idx

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

    let date = info.time;
    let filename = format!(
        "01-GTME-TMREC_{:02}{:02}{:04}_{:02}{:02}{:02}.gci", 
        date.month, date.day, date.year,
        date.hour, date.minute, date.second,
    );

    (bytes, filename)
}

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
