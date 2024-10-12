static MAPS: [&'static [u32]; 27] = [
    anim_maps::MARIO_DR_MARIO_SPECIAL_ANIM_MAP,
    anim_maps::FOX_FALCO_SPECIAL_ANIM_MAP,
    anim_maps::CAPTAIN_FALCON_GANONDORF_SPECIAL_ANIM_MAP,
    anim_maps::DONKEY_KONG_SPECIAL_ANIM_MAP,
    anim_maps::KIRBY_SPECIAL_ANIM_MAP,
    anim_maps::BOWSER_SPECIAL_ANIM_MAP,
    anim_maps::LINK_YOUNG_LINK_SPECIAL_ANIM_MAP,
    anim_maps::SHEIK_SPECIAL_ANIM_MAP,
    anim_maps::NESS_SPECIAL_ANIM_MAP,
    anim_maps::PEACH_SPECIAL_ANIM_MAP,
    anim_maps::ICE_CLIMBERS_SPECIAL_ANIM_MAP,
    anim_maps::ICE_CLIMBERS_SPECIAL_ANIM_MAP,
    anim_maps::PIKACHU_PICHU_SPECIAL_ANIM_MAP,
    anim_maps::SAMUS_SPECIAL_ANIM_MAP,
    anim_maps::YOSHI_SPECIAL_ANIM_MAP,
    anim_maps::JIGGLYPUFF_SPECIAL_ANIM_MAP,
    anim_maps::MEWTWO_SPECIAL_ANIM_MAP,
    anim_maps::LUIGI_SPECIAL_ANIM_MAP,
    anim_maps::MARTH_ROY_SPECIAL_ANIM_MAP,
    anim_maps::ZELDA_SPECIAL_ANIM_MAP,
    anim_maps::LINK_YOUNG_LINK_SPECIAL_ANIM_MAP,
    anim_maps::MARIO_DR_MARIO_SPECIAL_ANIM_MAP,
    anim_maps::FOX_FALCO_SPECIAL_ANIM_MAP,
    anim_maps::PIKACHU_PICHU_SPECIAL_ANIM_MAP,
    anim_maps::MR_GAME_AND_WATCH_SPECIAL_ANIM_MAP,
    anim_maps::CAPTAIN_FALCON_GANONDORF_SPECIAL_ANIM_MAP,
    anim_maps::MARTH_ROY_SPECIAL_ANIM_MAP,
];

fn main() {
    // read Start.dol from vanilla melee
    let dol = std::fs::read("Start.dol").unwrap();

    std::fs::write("src/fn_table.raw", &dol[0x3BF800..0x3C22A0]).unwrap();

    let char_offsets_list_offset = 0x003BE2E0;
    let offsets = &dol[char_offsets_list_offset..][..27*4];
    let mut out = Vec::new();
    for i in 0..27 {
        let len = MAPS[i].len();
        let mem_address = u32::from_be_bytes(offsets[i*4..][..4].try_into().unwrap());

        // heavy-handedly translate the mem address to a dol offset
        let mario_special_fn_offset = 0x003C4120;
        let dol_offset = mem_address 
            - u32::from_be_bytes(offsets[0..4].try_into().unwrap())
            + mario_special_fn_offset;

        println!("0x{:08x}: 0x{:04x} {:3}", out.len(), dol_offset, len);
        out.extend_from_slice(&dol[dol_offset as usize..][..len*0x20]);
    }
    std::fs::write("src/special_fn_table.raw", out.as_slice()).unwrap();
}

// ------------- copied from rwing ----------------

pub mod anim_options {
    pub const ANIM_BITCOUNT: u32 = 10;
    pub const FLAG_BITCOUNT: u32 = 14;

    pub const ANIM_MASK: u32   = 0b0000_0000_0000_0000_0000_0011_1111_1111;
    pub const FLAG_MASK: u32   = 0b0000_0000_1111_1111_1111_1100_0000_0000;
    pub const ACTION_MASK: u32 = 0b1111_1111_0000_0000_0000_0000_0000_0000;

    pub const TODO_ANIM      : u32 = 0;

    pub const INVISIBLE              : u32 = 1 << (ANIM_BITCOUNT + 0);
    pub const ROT_TO_VEL_Y           : u32 = 1 << (ANIM_BITCOUNT + 1); // rotate from y axis to vel (DamageFlyRoll)
    pub const ROT_TO_VEL_X           : u32 = 1 << (ANIM_BITCOUNT + 2); // rotate from x axis to vel (firefox, firebird, pk thunder) 
                                                                       // rotate from xy axis to vel (pikachu pichu)
    pub const SHIELD                 : u32 = 1 << (ANIM_BITCOUNT + 3);
    pub const RM_TRANSLATION         : u32 = 1 << (ANIM_BITCOUNT + 4);
    pub const OPTIONAL_TURN          : u32 = 1 << (ANIM_BITCOUNT + 5); // optional turning within an animation (most upbs)
    pub const INTERPOLATE_INTO       : u32 = 1 << (ANIM_BITCOUNT + 6); // animation blending (currently unimplemented)
    pub const CONTROL_STICK_SHIFT    : u32 = 1 << (ANIM_BITCOUNT + 7); // shield shifting
    pub const NO_CAMERA_TRACK        : u32 = 1 << (ANIM_BITCOUNT + 8); // 
    pub const DEBUG                  : u32 = 1 << (ANIM_BITCOUNT + 13);

    pub const CHAR_ACTION_1: u32 = 1 << (ANIM_BITCOUNT + FLAG_BITCOUNT);
    pub const CHAR_ACTION_2: u32 = 2 << (ANIM_BITCOUNT + FLAG_BITCOUNT);
    pub const CHAR_ACTION_3: u32 = 3 << (ANIM_BITCOUNT + FLAG_BITCOUNT);
    pub const CHAR_ACTION_4: u32 = 4 << (ANIM_BITCOUNT + FLAG_BITCOUNT);
    pub const CHAR_ACTION_5: u32 = 5 << (ANIM_BITCOUNT + FLAG_BITCOUNT);

    // fox / falco
    pub const SHINE: u32 = CHAR_ACTION_1;

    // peach
    pub const GOLF_CLUB: u32 = CHAR_ACTION_1;
    pub const FRYING_PAN: u32 = CHAR_ACTION_2;
    pub const TENNIS_RACKET: u32 = CHAR_ACTION_3;
    pub const PARASOL: u32 = CHAR_ACTION_4;
    pub const TOAD: u32 = CHAR_ACTION_5;

    // samus
    pub const MORPH_BALL: u32 = CHAR_ACTION_1;
    pub const SCREW_ATTACK: u32 = CHAR_ACTION_2;

    // game and watch
    //pub const SHOW_ATTACK_MODEL: u32 = CHAR_ACTION_1;
    
    // bowser and yoshi
    pub const SHELL: u32 = CHAR_ACTION_1;
}

// https://docs.google.com/spreadsheets/d/1Nu3hSc1U6apOhU4JIJaWRC4Lj0S1inN8BFsq3Y8cFjI

pub mod anim_maps {
    use super::anim_options::*;

    // SPECIAL ANIM MAPs ----------------------------------------------

    pub static MARTH_ROY_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // ShieldBreakerGroundStartCharge  => SpecialNStart     
        296, // ShieldBreakerGroundChargeLoop   => SpecialNLoop
        297, // ShieldBreakerGroundEarlyRelease => SpecialNEnd
        298, // ShieldBreakerGroundFullyCharged => SpecialNEnd
        299, // ShieldBreakerAirStartCharge     => SpecialAirNStart
        300, // ShieldBreakerAirChargeLoop      => SpecialAirNLoop
        301, // ShieldBreakerAirEarlyRelease    => SpecialAirNEnd
        302, // ShieldBreakerAirFullyCharged    => SpecialAirNEnd
        303, // DancingBlade1Ground             => SpecialS1
        304, // DancingBlade2UpGround           => SpecialS2Hi
        305, // DancingBlade2SideGround         => SpecialS2Lw
        RM_TRANSLATION | 306, // DancingBlade3UpGround           => SpecialS3Hi
        RM_TRANSLATION | 307, // DancingBlade3SideGround         => SpecialS3S
        RM_TRANSLATION | 308, // DancingBlade3DownGround         => SpecialS3Lw
        RM_TRANSLATION | 309, // DancingBlade4UpGround           => SpecialS4Hi
        RM_TRANSLATION | 310, // DancingBlade4SideGround         => SpecialS4S
        RM_TRANSLATION | 311, // DancingBlade4DownGround         => SpecialS4Lw
        312, // DancingBlade1Air                => SpecialAirS1
        313, // DancingBlade2UpAir              => SpecialAirS2Hi
        314, // DancingBlade2SideAir            => SpecialAirS2Lw
        315, // DancingBlade3UpAir              => SpecialAirS3Hi
        316, // DancingBlade3SideAir            => SpecialAirS3S
        317, // DancingBlade3DownAir            => SpecialAirS3Lw
        318, // DancingBlade4UpAir              => SpecialAirS4Hi
        319, // DancingBlade4SideAir            => SpecialAirS4S
        320, // DancingBlade4DownAir            => SpecialAirS4Lw
        OPTIONAL_TURN | RM_TRANSLATION | 321, // DolphinSlashGround              => SpecialHi
        OPTIONAL_TURN | RM_TRANSLATION | 322, // DolphinSlashAir                 => SpecialAirHi
        323, // CounterGround                   => SpecialLw
        324, // CounterGroundHit                => SpecialLwHit
        325, // CounterAir                      => SpecialAirLw
        326, // CounterAirHit                   => SpecialAirLwHit
    ];

    pub static FOX_FALCO_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // BlasterGroundStartup            => SpecialNStart
        296, // BlasterGroundLoop               => SpecialNLoop
        297, // BlasterGroundEnd                => SpecialNEnd
        298, // BlasterAirStartup               => SpecialAirNStart
        299, // BlasterAirLoop                  => SpecialAirNLoop
        300, // BlasterAirEnd                   => SpecialAirNEnd
        301, // IllusionGroundStartup           => SpecialSStart
        RM_TRANSLATION | 302, // IllusionGround                  => SpecialS
        RM_TRANSLATION | 303, // IllusionGroundEnd               => SpecialSEnd
        304, // IllusionStartupAir              => SpecialAirSStart
        RM_TRANSLATION | 305, // IllusionAir                     => SpecialAirS
        RM_TRANSLATION | 306, // IllusionAirEnd                  => SpecialAirSEnd
        307, // FireBirdGroundStartup           => SpecialHiHold
        308, // FireBirdAirStartup              => SpecialHiHoldAir
        ROT_TO_VEL_X | 309, // FireBirdGround                  => SpecialHi
        ROT_TO_VEL_X | 309, // FireBirdAir                     => SpecialHi
        310, // FireBirdGroundEnd               => SpecialHiLanding
        311, // FireBirdAirEnd                  => SpecialHiFall
        RM_TRANSLATION | 312, // FireBirdBounceEnd               => SpecialHiBound
        SHINE | 313, // ReflectorGroundStartup          => SpecialLwStart
        SHINE | 314, // ReflectorGroundLoop             => SpecialLwLoop
        SHINE | 315, // ReflectorGroundReflect          => SpecialLwHit
        316, // ReflectorGroundEnd              => SpecialLwEnd
        SHINE | 314, // ReflectorGroundChangeDirection  => SpecialLwLoop
        SHINE | 317, // ReflectorAirStartup             => SpecialAirLwStart
        SHINE | 318, // ReflectorAirLoop                => SpecialAirLwLoop
        SHINE | 319, // ReflectorAirReflect             => SpecialAirLwHit                 
        320, // ReflectorAirEnd                 => SpecialAirLwEnd   
        SHINE | 314, // ReflectorAirChangeDirection     => SpecialLwLoop
        321, // SmashTauntRightStartup          => AppealSStartR
        322, // SmashTauntLeftStartup           => AppealSStartL
        323, // SmashTauntRightRise             => AppealSR
        324, // SmashTauntLeftRise              => AppealSL
        325, // SmashTauntRightFinish           => AppealSEndR
        326, // SmashTauntLeftFinish            => AppealSEndL
    ];

    pub static PEACH_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // Float                
        RM_TRANSLATION | 296, // FloatEndForward      
        RM_TRANSLATION | 297, // FloatEndBackward    
        068, // FloatNair            
        069, // FloatFair            
        070, // FloatBair            
        071, // FloatUair            
        072, // FloatDair            
        GOLF_CLUB | 298, // SideSmashGolfClub    
        FRYING_PAN | 299, // SideSmashFryingPan   
        TENNIS_RACKET | 300, // SideSmashTennisRacket
        301, // VegetableGround      
        301, // VegetableAir         
        302, // BomberGroundStartup  
        303, // BomberGroundEnd      
        TODO_ANIM, // unused
        305, // BomberAirStartup      
        306, // BomberAirEnd            
        307, // BomberAirHit         
        304, // BomberAir            
        OPTIONAL_TURN | RM_TRANSLATION | PARASOL | 308, // ParasolGroundStart   
        OPTIONAL_TURN | RM_TRANSLATION | PARASOL | 309, // unused
        OPTIONAL_TURN | RM_TRANSLATION | PARASOL | 310, // ParasolAirStart      
        OPTIONAL_TURN | RM_TRANSLATION | PARASOL | 311, // unused
        TOAD | 312, // ToadGround           
        TOAD | 313, // ToadGroundAttack     
        TOAD | 314, // ToadAir              
        TOAD | 315, // ToadAirAttack        
        PARASOL | 316, // ParasolOpening       
        PARASOL | 317, // ParasolOpen          
    ];

    pub static CAPTAIN_FALCON_GANONDORF_SPECIAL_ANIM_MAP: &'static [u32] = &[
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        RM_TRANSLATION | 301, // FalconPunchGround               
        RM_TRANSLATION | 302, // FalconPunchAir                  
        RM_TRANSLATION | 303, // RaptorBoostGround               
        RM_TRANSLATION | 304, // RaptorBoostGroundHit            
        RM_TRANSLATION | 305, // RaptorBoostAir                  
        RM_TRANSLATION | 306, // RaptorBoostAirHit               
        OPTIONAL_TURN | RM_TRANSLATION | 307, // FalconDiveGround                
        OPTIONAL_TURN | RM_TRANSLATION | 308, // FalconDiveAir                   
        309, // FalconDiveCatch                 
        310, // FalconDiveEnding                
        RM_TRANSLATION | 311, // FalconKickGround                
        312, // FalconKickGroundEndingOnGround  
        RM_TRANSLATION | 313, // FalconKickAir                   
        314, // FalconKickAirEndingOnGround     
        RM_TRANSLATION | 315, // FalconKickAirEndingInAir        
        316, // FalconKickGroundEndingInAir     
        RM_TRANSLATION | 317, // FalconKickHitWall               
    ];

    pub static SHEIK_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // NeedleStormGroundStartCharge "SpecialNStart",
        296, // NeedleStormGroundChargeLoop  "SpecialNLoop",
        297, // NeedleStormGroundEndCharge   "SpecialNCancel",
        298, // NeedleStormGroundFire        "SpecialNEnd",
        299, // NeedleStormAirStartCharge    "SpecialAirNStart",
        300, // NeedleStormAirChargeLoop     "SpecialAirNLoop",
        301, // NeedleStormAirEndCharge      "SpecialAirNCancel",
        302, // NeedleStormAirFire           "SpecialAirNEnd",
        303, // ChainGroundStartup           "SpecialSStart",
        305, // ChainGroundLoop              "SpecialS",
        304, // ChainGroundEnd               "SpecialSEnd",
        306, // ChainAirStartup              "SpecialAirSStart",
        308, // ChainAirLoop                 "SpecialAirS",
        307, // ChainAirEnd                  "SpecialAirSEnd",
        309, // VanishGroundStartup          "SpecialHiStart",
        INVISIBLE | 310, // VanishGroundDisappear        "SpecialHi", // ????????
        310, // VanishGroundReappear         "SpecialHi", // ????????
        311, // VanishAirStartup             "SpecialAirHiStart",
        INVISIBLE | 312, // VanishAirDisappear           "SpecialAirHi", // ????????
        312, // VanishAirReappear            "SpecialAirHi", // ????????
        313, // TransformGround              "SpecialLw",
        314, // TransformGroundEnding        "SpecialLw2", // ?????
        315, // TransformAir                 "SpecialAirLw",
        316, // TransformAirEnding           "SpecialAirLw2", // ?????
    ];

    pub static SAMUS_SPECIAL_ANIM_MAP: &'static [u32] = &[
        MORPH_BALL | 295, // BombJumpGround          SpecialLw
        MORPH_BALL | 296, // BombJumpAir             SpecialAirLw
        297, // ChargeShotGroundStart   SpecialNStart
        298, // ChargeShotGroundLoop    SpecialNHold
        299, // ChargeShotGroundEnd     SpecialNCancel
        300, // ChargeShotGroundFire    SpecialN
        301, // ChargeShotAirStart      SpecialAirNStart
        302, // ChargeShotAirFire       SpecialAirN
        303, // MissileGround           SpecialS
        304, // MissileSmashGround      Special
        305, // MissileAir              SpecialAirS
        306, // MissileSmashAir         SpecialAir
        OPTIONAL_TURN | RM_TRANSLATION | SCREW_ATTACK | 307, // ScrewAttackGround       SpecialHi
        OPTIONAL_TURN | RM_TRANSLATION | SCREW_ATTACK | 308, // ScrewAttackAir          SpecialAirHi
        MORPH_BALL | 309, // BombEndGround           SpecialLw
        MORPH_BALL | 310, // BombAir                 SpecialAirLw
        311, // Zair                    AirCatch
        312, // ZairCatch               AirCatchHit 
    ];

    pub static MARIO_DR_MARIO_SPECIAL_ANIM_MAP: &'static [u32] = &[
        239, // TauntR                AppealR               IDK
        TODO_ANIM, // Unknown342            
        295, // MegavitaminGround     SpecialN
        296, // MegavitaminAir        SpecialNAir
        297, // SuperSheetGround      SpecialS
        298, // SuperSheetAir         SpecialSAir
        OPTIONAL_TURN | RM_TRANSLATION | 299, // SuperJumpPunchGround  SpecialHi
        OPTIONAL_TURN | RM_TRANSLATION | 300, // SuperJumpPunchAir     SpecialAirHi
        301, // TornadoGround         SpecialLw
        302, // TornadoAir            SpecialAirLw
    ];

    pub static DONKEY_KONG_SPECIAL_ANIM_MAP: &'static [u32] = &[
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused
        TODO_ANIM, // unused

        295, // KongKarryWait                       HeavyWait
        296, // KongKarryWalkSlow                   HeavyWalkSlow
        297, // KongKarryWalkMiddle                 HeavyWalkMiddle
        298, // KongKarryWalkFast                   HeavyWalkFast
        299, // KongKarryTurn                       HeavyTurn
        300, // KongKarryJumpSquat                  HeavyWait
        301, // KongKarryFall                       HeavyWait
        302, // KongKarryJump                       HeavyWait
        303, // KongKarryLanding                    HeavyWait       
             
        TODO_ANIM,
        // bunch of unused animations????

        315, // KongKarryGroundThrowForward         ThrowFF
        316, // KongKarryGroundThrowBackward        ThrowFB
        317, // KongKarryGroundThrowUp              ThrowFHi
        318, // KongKarryGroundThrowDown            ThrowFLw
        315, // KongKarryAirThrowForward            ThrowFF
        316, // KongKarryAirThrowBackward           ThrowFB
        317, // KongKarryAirThrowUp                 ThrowFHi
        318, // KongKarryAirThrowDown               ThrowFLw
        319, // GiantPunchGroundChargeStartup       SpecialNStart       
        320, // GiantPunchGroundChargeLoop          SpecialNLoop
        321, // GiantPunchGroundChargeStop          SpecialNCansel
        322, // GiantPunchGroundEarlyPunch          SpecialN
        323, // GiantPunchGroundFullChargePunch     SpecialN
        324, // GiantPunchAirChargeStartup          SpecialAirNStart
        325, // GiantPunchAirChargeLoop             SpecialAirNLoop
        326, // GiantPunchAirChargeStop             SpecialAirNCancel
        327, // GiantPunchAirEarlyPunch             SpecialAirN
        328, // GiantPunchAirFullChargePunch        SpecialAirN
        329, // HeadbuttGround                      SpecialS
        330, // HeadbuttAir                         SpecialAirS
        331, // SpinningKongGround                  SpecialHi
        332, // SpinningKongAir                     SpecialAirHi
        333, // HandSlapStartup                     SpecialLwStart
        334, // HandSlapLoop                        SpecialLwLoop
        335, // HandSlapEnd                         SpecialLwEnd
    ];

    pub static PIKACHU_PICHU_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295,            // ThunderJoltGround        SpecialN            
        296,            // ThunderJoltAir           SpecialAirN
        297,            // SkullBashGroundStartup   SpecialSStart
        298,            // SkullBashGroundCharge    SpecialSHold
        TODO_ANIM,      // Unknown345               
        301,            // SkullBashGroundLanding   SpecialSEnd
        299,            // SkullBashGroundTakeoff   SpecialS
        302,            // SkullBashAirStartup      SpecialAirSStart
        303,            // SkullBashAirCharge       SpecialAirSHold
        304,            // SkullBashAir             SpecialS
        305,            // SkullBashAirEnd          SpecialAirSEnd
        304,            // SkullBashAirTakeoff      SpecialS
        306,            // AgilityGroundStartup     SpecialHiStart
        ROT_TO_VEL_X | ROT_TO_VEL_Y | 307, // AgilityGround            SpecialHiStart
        308,            // AgilityGroundEnd         SpecialHiEnd
        309,            // AgilityAirStartup        SpecialAirHiStart
        ROT_TO_VEL_X | ROT_TO_VEL_Y | 310, // AgilityAir               SpecialAirHiStart
        311,            // AgilityAirEnd            SpecialAirHiEnd
        312,            // ThunderGroundStartup     SpecialLwStart
        313,            // ThunderGround            SpecialLwLoop
        314,            // ThunderGroundHit         SpecialLwLoop
        315,            // ThunderGroundEnd         SpecialLwEnd
        316,            // ThunderAirStartup        SpecialAirLwStart
        317,            // ThunderAir               SpecialAirLwLoop
        318,            // ThunderAirHit            SpecialAirLwLoop    
        319,            // ThunderAirEnd            SpecialAirLwEnd  
    ];

    pub static LUIGI_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295,        // FireballGround                       SpecialN
        296,        // FireballAir                          SpecialAirN
        297,        // GreenMissileGroundStartup            SpecialSStart
        298,        // GreenMissileGroundCharge             SpecialSHold
        TODO_ANIM,  // Unknown345                           
        302,        // GreenMissileGroundLanding            SpecialSEnd
        300,        // GreenMissileGroundTakeoff            SpecialS
        301,        // GreenMissileGroundTakeoffMisfire     SpecialS
        303,        // GreenMissileAirStartup               SpecialAirSStart
        304,        // GreenMissileAirCharge                SpecialAirSHold
        305,        // GreenMissileAir                      SpecialS
        307,        // GreenMissileAirEnd                   SpecialAirSEnd
        305,        // GreenMissileAirTakeoff               SpecialS
        306,        // GreenMissileAirTakeoffMisfire        SpecialS
        OPTIONAL_TURN | RM_TRANSLATION | 308,        // SuperJumpPunchGround                 SpecialHi
        OPTIONAL_TURN | RM_TRANSLATION | 309,        // SuperJumpPunchAir                    SpecialAirHi
        310,        // CycloneGround                        SpecialLw
        311,        // CycloneAir                           SpecialAirLw
    ];

    pub static MR_GAME_AND_WATCH_SPECIAL_ANIM_MAP: &'static [u32] = &[
        046,            // Jab                      Attack11
        047,            // Jab2                     Attack12
        050,            // RapidJabs                Attack100Loop
        051,            // RapidJabsEnd             Attack100End
        059,            // DownTilt                 AttackLw3
        062,            // SideSmash                AttackS4S
        068,            // Nair                     AttackAirN
        070,            // Bair                     AttackAirB
        071,            // Uair                     AttackAirHi
        073,            // NairLanding              LandingAirN
        074,            // BairLanding              LandingAirB
        076,            // UairLanding              LandingAirHi
        295,            // ChefGround               SpecialN             
        296,            // ChefAir                  SpecialAirN
        297,            // Judgment1Ground          SpecialS
        298,            // Judgment2Ground          SpecialS
        299,            // Judgment3Ground          SpecialS
        300,            // Judgment4Ground          SpecialS
        301,            // Judgment5Ground          SpecialS
        302,            // Judgment6Ground          SpecialS
        303,            // Judgment7Ground          SpecialS
        304,            // Judgment8Ground          SpecialS
        305,            // Judgment9Ground          SpecialS
        306,            // Judgment1Air             SpecialAirS
        307,            // Judgment2Air             SpecialAirS
        308,            // Judgment3Air             SpecialAirS
        309,            // Judgment4Air             SpecialAirS
        310,            // Judgment5Air             SpecialAirS
        311,            // Judgment6Air             SpecialAirS
        312,            // Judgment7Air             SpecialAirS
        313,            // Judgment8Air             SpecialAirS
        314,            // Judgment9Air             SpecialAirS
        OPTIONAL_TURN | RM_TRANSLATION | 315, // FireGround         SpecialHi
        OPTIONAL_TURN | RM_TRANSLATION | 316, // FireAir            SpecialAirHi
        317,            // OilPanicGround           SpecialLw
        318,            // OilPanicGroundAbsorb     SpecialLwCatch
        319,            // OilPanicGroundSpill      SpecialLwShoot
        320,            // OilPanicAir              SpecialAirLw
        321,            // OilPanicAirAbsorb        SpecialAirLwCatch
        322,            // OilPanicAirSpill         SpecialAirLwShoot
    ];

    pub static MEWTWO_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295,                // ShadowBallGroundStartCharge         SpecialNStart
        296,                // ShadowBallGroundChargeLoop          SpecialNLoop
        297,                // ShadowBallGroundFullyCharged        SpecialNLoop
        298,                // ShadowBallGroundEndCharge           SpecialNCancel
        299,                // ShadowBallGroundFire                SpecialNEnd
        300,                // ShadowBallAirStartCharge            SpecialAirNStart
        301,                // ShadowBallAirChargeLoop             SpecialAirNLoop
        302,                // ShadowBallAirFullyCharged           SpecialAirNLoop
        303,                // ShadowBallAirEndCharge              SpecialAirNCancel
        304,                // ShadowBallAirFire                   SpecialAirNEnd
        305,                // ConfusionGround                     SpecialS
        306,                // ConfusionAir                        SpecialAirS      
        307,                // TeleportGroundStartup               SpecialHiStart
        INVISIBLE | 308,    // TeleportGroundDisappear             SpecialHi
        309,                // TeleportGroundReappear              SpecialHiLost
        310,                // TeleportAirStartup                  SpecialAirStart
        INVISIBLE | 311,    // TeleportAirDisappear                SpecialAirHi
        309,                // TeleportAirReappear                 SpecialHiLost
        312,                // DisableGround                       SpecialLw
        313,                // DisableAir                          SpecialAirLw
    ];

    pub static BOWSER_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // FireBreathGroundStartup 
        296, // FireBreathGroundLoop    
        297, // FireBreathGroundEnd     
        298, // FireBreathAirStartup    
        299, // FireBreathAirLoop       
        300, // FireBreathAirEnd        
        301, // KoopaKlawGround         
        302, // KoopaKlawGroundGrab     
        303, // KoopaKlawGroundPummel   
        303, // KoopaKlawGroundWait     
        304, // KoopaKlawGroundThrowF   
        305, // KoopaKlawGroundThrowB   
        306, // KoopaKlawAir            
        307, // KoopaKlawAirGrab        
        308, // KoopaKlawAirPummel      
        308, // KoopaKlawAirWait        
        309, // KoopaKlawAirThrowF      
        310, // KoopaKlawAirThrowB      
        SHELL | 311, // WhirlingFortressGround  
        SHELL | 312, // WhirlingFortressAir     
        313, // BombGroundBegin         
        314, // BombAir                 
        315, // BombLand                
    ];

    pub static JIGGLYPUFF_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // Jump2                           JumpAerialF1
        296, // Jump3                           JumpAerialF2
        297, // Jump4                           JumpAerialF3
        298, // Jump5                           JumpAerialF4
        299, // Jump6                           JumpAerialF5
        300, // RolloutGroundStartChargeRight   SpecialNStartR
        301, // RolloutGroundStartChargeLeft    SpecialNStartL
        302, // RolloutGroundChargeLoop         SpecialN
        303, // RolloutGroundFullyCharged       SpecialN
        304, // RolloutGroundChargeRelease      SpecialN
        305, // RolloutGroundStartTurn          SpecialN
        306, // RolloutGroundEndRight           SpecialNEndR
        307, // RolloutGroundEndLeft            SpecialNEndL
        308, // RolloutAirStartChargeRight      SpecialAirNStartR
        309, // RolloutAirStartChargeLeft       SpecialAirNStartL
        310, // RolloutAirChargeLoop            SpecialN
        311, // RolloutAirFullyCharged          SpecialN
        312, // RolloutAirChargeRelease         SpecialN
        313, // Unknown359                      SpecialN
        314, // RolloutAirEndRight              SpecialAirNEndR
        315, // RolloutAirEndLeft               SpecialAirNEndL
        316, // RolloutHit                      SpecialN
        317, // PoundGround                     SpecialS
        318, // PoundAir                        SpecialAirS
        319, // SingGroundLeft                  SpecialHiL
        320, // SingAirLeft                     SpecialAirHiL
        321, // SingGroundRight                 SpecialHiR
        322, // SingAirRight                    SpecialAirHiR
        323, // RestGroundLeft                  SpecialLwL
        324, // RestAirLeft                     SpecialAirLwL
        325, // RestGroundRight                 SpecialLwR
        326, // RestAirRight                    SpecialAirLwR      
    ];

    pub static LINK_YOUNG_LINK_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // SideSmash2                  AttackS42          
        239, // TauntL for YL, Unused for link
        240, // TauntR for YL, Unused for link
        296, // BowGroundCharge             SpecialNStart
        297, // BowGroundFullyCharged       SpecialNLoop
        298, // BowGroundFire               SpecialNEnd
        299, // BowAirCharge                SpecialAirNStart
        300, // BowAirFullyCharged          SpecialAirNLoop
        301, // BowAirFire                  SpecialAirNEnd
        302, // BoomerangGroundThrow        SpecialS1
        303, // BoomerangGroundCatch        SpecialS2
        304, // BoomerangGroundThrowEmpty   SpecialS1
        305, // BoomerangAirThrow           SpecialAirS1
        306, // BoomerangAirCatch           SpecialAirS2
        307, // BoomerangAirThrowEmpty      SpecialAirS1
        308, // SpinAttackGround            SpecialHi
        309, // SpinAttackAir               SpecialAirHi
        310, // BombGround                  SpecialLw
        311, // BombAir                     SpecialAirLw
        312, // Zair                        AirCatch
        313, // ZairCatch                   AirCatchHit
    ];

    pub static KIRBY_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // Jump2                                                JumpAerialF1
        296, // Jump3                                                JumpAerialF2
        297, // Jump4                                                JumpAerialF3
        298, // Jump5                                                JumpAerialF4
        299, // Jump6                                                JumpAerialF5
        300, // Jump2WithHat                                         JumpAerialF1Met
        301, // Jump3WithHat                                         JumpAerialF2Met
        302, // Jump4WithHat                                         JumpAerialF3Met
        303, // Jump5WithHat                                         JumpAerialF4Met
        304, // Jump6WithHat                                         JumpAerialF5Met
        RM_TRANSLATION | 052, // DashAttackGround                                     
        RM_TRANSLATION | 052, // DashAttackAir                                        
        305, // SwallowGroundStartup                                 
        306, // SwallowGroundLoop                                    
        307, // SwallowGroundEnd                                     
        308, // SwallowGroundCapture                                 
        TODO_ANIM, // Unknown357                                           
        309, // SwallowGroundCaptured                                
        310, // SwallowGroundCaptureWait                             
        311, // SwallowCaptureWalkSlow                               
        312, // SwallowCaptureWalkMiddle                             
        313, // SwallowCaptureWalkFast                               
        317, // SwallowGroundCaptureTurn                             
        314, // SwallowCaptureJumpSquat                              
        315, // SwallowCaptureJump                                   
        316, // SwallowCaptureLanding                                
        318, // SwallowGroundDigest                                  
        TODO_ANIM, // Unknown368                                           
        319, // SwallowGroundSpit                                    
        TODO_ANIM, // Unknown370                                           
        320, // SwallowAirStartup                                    
        321, // SwallowAirLoop                                       
        307, // SwallowAirEnd                                        
        308, // SwallowAirCapture                                    
        TODO_ANIM, // Unknown375                                           
        309, // SwallowAirCaptured                                   
        310, // SwallowAirCaptureWait                                
        318, // SwallowAirDigest                                     
        TODO_ANIM, // Unknown379                                           
        319, // SwallowAirSpit                                       
        TODO_ANIM, // Unknown381                                           
        317, // SwallowAirCaptureTurn                                
        322, // HammerGround                                         
        323, // HammerAir                                            
        OPTIONAL_TURN | RM_TRANSLATION | 324, // FinalCutterGroundStartup                             
        325, // Unknown386                                           
        326, // Unknown387                                           
        OPTIONAL_TURN | RM_TRANSLATION | 327, // FinalCutterGroundEnd                                 
        OPTIONAL_TURN | RM_TRANSLATION | 328, // FinalCutterAirStartup                                
        OPTIONAL_TURN | RM_TRANSLATION | 329, // FinalCutterAirApex                                   
        OPTIONAL_TURN | RM_TRANSLATION | 330, // FinalCutterSwordDescent                              
        OPTIONAL_TURN | RM_TRANSLATION | 331, // FinalCutterAirEnd                                    
        332, // StoneGroundStartup                                   
        333, // StoneGround                                          
        334, // StoneGroundEnd                                       
        335, // StoneAirStartup                                      
        336, // StoneAir                                             
        337, // StoneAirEnd                                          
        338, // MarioFireballGround                                  
        339, // MarioFireballAir                                     
        340, // LinkBowGroundCharge                                  
        341, // LinkBowGroundFullyCharged                            
        342, // LinkBowGroundFire                                    
        343, // LinkBowAirCharge                                     
        344, // LinkBowAirFullyCharged                               
        345, // LinkBowAirFire                                       
        346, // SamusChargeShotGroundStart                           
        347, // SamusChargeShotGroundLoop                            
        348, // SamusChargeShotGroundEnd                             
        349, // SamusChargeShotGroundFire                            
        350, // SamusChargeShotAirStart                              
        351, // SamusChargeShotAirFire                               
        352, // YoshiEggLayGround                                    
        353, // YoshiEggLayGroundCaptureStart                        
        TODO_ANIM, // Unknown415                                           
        354, // YoshiEggLayGroundCapture                             
        TODO_ANIM, // Unknown417                                           
        355, // YoshiEggLayAir                                       
        356, // YoshiEggLayAirCaptureStart                           
        TODO_ANIM, // Unknown420                                           
        357, // YoshiEggLayAirCapture                                
        TODO_ANIM, // Unknown422                                           
        358, // FoxBlasterGroundStartup                              
        359, // FoxBlasterGroundLoop                                 
        360, // FoxBlasterGroundEnd                                  
        361, // FoxBlasterAirStartup                                 
        362, // FoxBlasterAirLoop                                    
        363, // FoxBlasterAirEnd                                     
        364, // PikachuThunderJoltGround                             
        365, // PikachuThunderJoltAir                                
        366, // LuigiFireballGround                                  
        367, // LuigiFireballAir                                     
        368, // FalconFalconPunchGround                              
        369, // FalconFalconPunchAir                                 
        370, // NessPKFlashGroundStartup                             
        371, // NessPKFlashGroundCharge                              
        372, // NessPKFlashGroundExplode                             
        373, // NessPKFlashGroundEnd                                 
        374, // NessPKFlashAirStartup                                
        375, // NessPKFlashAirCharge                                 
        376, // NessPKFlashAirExplode                                
        377, // NessPKFlashAirEnd                                    
        378, // BowserFireBreathGroundStart                          
        379, // BowserFireBreathGroundLoop                           
        380, // BowserFireBreathGroundEnd                            
        381, // BowserFireBreathAirStart                             
        382, // BowserFireBreathAirLoop                              
        383, // BowserFireBreathAirEnd                               
        384, // PeachToadGround                                      
        385, // PeachToadGroundAttack                                
        386, // PeachToadAir                                         
        387, // PeachToadAirAttack                                   
        388, // IceClimbersIceShotGround                             
        389, // IceClimbersIceShotAir                                
        390, // DKGiantPunchGroundChargeStartup                      
        391, // DKGiantPunchGroundChargeLoop                         
        392, // DKGiantPunchGroundChargeStop                         
        393, // DKGiantPunchGroundEarlyPunch                         
        394, // DKGiantPunchGroundFullChargePunch                    
        395, // DKGiantPunchAirChargeStartup                         
        396, // DKGiantPunchAirChargeLoop                            
        397, // DKGiantPunchAirChargeStop                            
        398, // DKGiantPunchAirEarlyPunch                            
        399, // DKGiantPunchAirFullChargePunch                       
        400, // ZeldaNayrusLoveGround                                
        401, // ZeldaNayrusLoveAir                                   
        402, // SheikNeedleStormGroundStartCharge                    
        403, // SheikNeedleStormGroundChargeLoop                     
        404, // SheikNeedleStormGroundEndCharge                      
        405, // SheikNeedleStormGroundFire                           
        406, // SheikNeedleStormAirStartCharge                       
        407, // SheikNeedleStormAirChargeLoop                        
        408, // SheikNeedleStormAirEndCharge                         
        409, // SheikNeedleStormAirFire                              
        410, // JigglypuffRolloutGroundStartChargeRight              
        411, // JigglypuffRolloutGroundStartChargeLeft               
        412, // JigglypuffRolloutGroundChargeLoop                    
        413, // JigglypuffRolloutGroundFullyCharged                  
        414, // JigglypuffRolloutGroundChargeRelease                 
        415, // JigglypuffRolloutGroundStartTurn                     
        416, // JigglypuffRolloutGroundEndRight                      
        417, // JigglypuffRolloutGroundEndLeft                       
        418, // JigglypuffRolloutAirStartChargeRight                 
        419, // JigglypuffRolloutAirStartChargeLeft                  
        420, // JigglypuffRolloutAirChargeLoop                       
        421, // JigglypuffRolloutAirFullyCharged                     
        422, // JigglypuffRolloutAirChargeRelease                    
        423, // Unknown488                                           
        424, // JigglypuffRolloutAirEndRight                         
        425, // JigglypuffRolloutAirEndLeft                          
        426, // JigglypuffRolloutHit                                 
        427, // MarthShieldBreakerGroundStartCharge                  
        428, // MarthShieldBreakerGroundChargeLoop                   
        429, // MarthShieldBreakerGroundEarlyRelease                 
        430, // MarthShieldBreakerGroundFullyCharged                 
        431, // MarthShieldBreakerAirStartCharge                     
        432, // MarthShieldBreakerAirChargeLoop                      
        433, // MarthShieldBreakerAirEarlyRelease                    
        434, // MarthShieldBreakerAirFullyCharged                    
        435, // MewtwoShadowBallGroundStartCharge                    
        436, // MewtwoShadowBallGroundChargeLoop                     
        437, // MewtwoShadowBallGroundFullyCharged                   
        438, // MewtwoShadowBallGroundEndCharge                      
        439, // MewtwoShadowBallGroundFire                           
        440, // MewtwoShadowBallAirStartCharge                       
        441, // MewtwoShadowBallAirChargeLoop                        
        442, // MewtwoShadowBallAirFullyCharged                      
        443, // MewtwoShadowBallAirEndCharge                         
        444, // MewtwoShadowBallAirFire                              
        445, // GameandWatchOilPanicGround                           
        446, // GameandWatchOilPanicAir                              
        447, // DocMegavitaminGround                                 
        448, // DocMegavitaminAir                                    
        449, // YoungLinkFireBowGroundCharge                         
        450, // YoungLinkFireBowGroundFullyCharged                   
        451, // YoungLinkFireBowGroundFire                           
        452, // YoungLinkFireBowAirCharge                            
        453, // YoungLinkFireBowAirFullyCharged                      
        454, // YoungLinkFireBowAirFire                              
        455, // FalcoBlasterGroundStartup                            
        456, // FalcoBlasterGroundLoop                               
        457, // FalcoBlasterGroundEnd                                
        458, // FalcoBlasterAirStartup                               
        459, // FalcoBlasterAirLoop                                  
        460, // FalcoBlasterAirEnd                                   
        461, // PichuThunderJoltGround                               
        462, // PichuThunderJoltAir                                  
        463, // GanonWarlockPunchGround                              
        464, // GanonWarlockPunchAir                                 
        465, // RoyFlareBladeGroundStartCharge                       
        466, // RoyFlareBladeGroundChargeLoop                        
        467, // RoyFlareBladeGroundEarlyRelease                      
        468, // RoyFlareBladeGroundFullyCharged                      
        469, // RoyFlareBladeAirStartCharge                          
        470, // RoyFlareBladeAirChargeLoop                           
        471, // RoyFlareBladeAirEarlyRelease                         
        472, // RoyFlareBladeAirFullyCharged                         
    ];

    pub static NESS_SPECIAL_ANIM_MAP: &'static [u32] = &[
        062, // SideSmash                 
        295, // UpSmash                   
        295, // UpSmashCharge             
        295, // UpSmashCharged            
        297, // DownSmash                 
        297, // DownSmashCharge           
        297, // DownSmashCharged          
        299, // PKFlashGroundStartup      
        300, // PKFlashGroundCharge       
        301, // PKFlashGroundExplode      
        302, // PKFlashGroundEnd          
        303, // PKFlashAirStartup         
        304, // PKFlashAirCharge          
        305, // PKFlashAirExplode         
        306, // PKFlashAirEnd             
        307, // PKFireGround              
        308, // PKFireAir                 
        309, // PKThunderGroundStartup    
        310, // PKThunderGround           
        311, // PKThunderGroundEnd        
        ROT_TO_VEL_X | 312, // PKThunderGroundHit        
        313, // PKThunderAirStartup       
        314, // PKThunderAir              
        315, // PKThunderAirEnd           
        ROT_TO_VEL_X | 316, // PKThunderAirHit           
        317, // PKThunderAirHitWall       
        318, // PSIMagnetGroundStartup    
        319, // PSIMagnetGroundLoop       
        320, // PSIMagnetGroundAbsorb     
        321, // PSIMagnetGroundEnd        
        TODO_ANIM, // Unknown371                
        322, // PSIMagnetAirStartup       
        323, // PSIMagnetAirLoop          
        324, // PSIMagnetAirAbsorb        
        325, // PSIMagnetAirEnd           
        TODO_ANIM, // Unknown376                
    ];

    pub static ICE_CLIMBERS_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // IceShotGround                       
        296, // IceShotAir                          
        297, // PopoSquallHammerGroundSolo          
        298, // PopoSquallHammerGroundTogether      
        299, // PopoSquallHammerAirSolo             
        300, // PopoSquallHammerAirTogether         
        301, // PopoBelayGroundStartup              
        302, // PopoBelayGroundCatapultingNana      
        303, // Unknown349                          
        304, // PopoBelayGroundFailedCatapulting    
        305, // PopoBelayGroundFailedCatapultingEnd 
        306, // PopoBelayAirStartup                 
        307, // PopoBelayAirCatapultingNana         
        308, // PopoBelayCatapulting                
        309, // PopoBelayAirFailedCatapulting       
        310, // PopoBelayAirFailedCatapultingEnd    
        311, // BlizzardGround                      
        312, // BlizzardAir                         
        298, // NanaSquallHammerGroundTogether      IDK
        300, // NanaSquallHammerAirTogether         IDK
        306, // NanaBelayCatapultStartup            IDK
        310, // NanaBelayGroundCatapultEnd          IDK
        TODO_ANIM, // Unknown363                          
        TODO_ANIM, // Unknown364                          
        302, // NanaBelayCatapulting                IDK
    ];

    pub static ZELDA_SPECIAL_ANIM_MAP: &'static [u32] = &[
        295, // NayrusLoveGround           
        296, // NayrusLoveAir              
        297, // DinsFireGroundStartup      
        298, // DinsFireGroundTravel       
        299, // DinsFireGroundExplode      
        300, // DinsFireAirStartup         
        301, // DinsFireAirTravel          
        302, // DinsFireAirExplode         
        303, // FaroresWindGround          
        TODO_ANIM, // FaroresWindGroundDisappear 
        304, // FaroresWindGroundReappear  
        305, // FaroresWindAir             
        TODO_ANIM, // FaroresWindAirDisappear    
        306, // FaroresWindAirReappear     
        307, // TransformGround            
        308, // TransformGroundEnding      
        309, // TransformAir               
        310, // TransformAirEnding         
    ];

    pub static YOSHI_SPECIAL_ANIM_MAP: &'static [u32] = &[
        TODO_ANIM, // BufferedShieldStartup        
        TODO_ANIM, // ShieldHold                   
        TODO_ANIM, // ShieldRelease                
        TODO_ANIM, // ShieldDamage                 
        TODO_ANIM, // ShieldStartup                
        295, // EggLayGround                 
        296, // EggLayGroundCaptureStart     
        TODO_ANIM, // Unknown348                   
        297, // EggLayGroundCapture          
        TODO_ANIM, // Unknown350                   
        298, // EggLayAir                    
        299, // EggLayAirCaptureStart        
        TODO_ANIM, // Unknown353                   
        300, // EggLayAirCapture             
        TODO_ANIM, // Unknown                      
        301, // EggRollGroundStartup         
        SHELL | 302, // EggRollGround                
        SHELL | 303, // EggRollGroundChangeDirection 
        304, // EggRollGroundEnd             
        305, // EggRollAirStart              
        SHELL | 306, // EggRollAir                   
        SHELL | 307, // EggRollBounce                
        308, // EggRollAirEnd                
        309, // EggThrowGround               
        310, // EggThrowAir                  
        RM_TRANSLATION | 311, // BombGround                   
        312, // BombLand                     
        313, // BombAir                      
    ];
}
