// half open range 44..70 (Attack11..LandingAirN) 
pub const ATTACK_RANGE_START: usize = 44;
pub const ATTACK_RANGE_END: usize = 70;

// indexed by character, then by state_id - ATTACK_RANGE_START
pub const ATTACK_HITBOXES: &[&[std::ops::Range<u32>]] = &[
    // Mario
    &[
        2..4, // Attack11
        2..4, // Attack12
        4..9, // Attack13
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        6..26, // AttackDash
        5..8, // AttackS3Hi
        0..4294967295, // 
        5..8, // AttackS3S
        0..4294967295, // 
        5..8, // AttackS3Lw
        4..13, // AttackHi3
        5..9, // AttackLw3
        12..17, // AttackS4Hi
        0..4294967295, // 
        12..17, // AttackS4S
        0..4294967295, // 
        12..17, // AttackS4Lw
        9..12, // AttackHi4
        5..15, // AttackLw4
        3..33, // AttackAirN
        18..23, // AttackAirF
        6..18, // AttackAirB
        4..10, // AttackAirHi
        10..30, // AttackAirLw
    ],
    // Fox
    &[
        2..4, // Attack11
        2..4, // Attack12
        0..4294967295, // 
        0..4294967295, // Attack100Start
        0..4294967295, // Attack100Loop
        0..4294967295, // Attack100End
        4..18, // AttackDash
        0..4294967295, // AttackS3Hi
        0..4294967295, // AttackS3HiS
        5..9, // AttackS3S
        0..4294967295, // AttackS3LwS
        0..4294967295, // AttackS3Lw
        5..12, // AttackHi3
        7..10, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        12..23, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        7..18, // AttackHi4
        6..11, // AttackLw4
        4..32, // AttackAirN
        6..46, // AttackAirF
        4..20, // AttackAirB
        8..15, // AttackAirHi
        5..26, // AttackAirLw
    ],
    // CaptainFalcon
    &[
        3..6, // Attack11
        4..7, // Attack12
        5..12, // Attack13
        0..4294967295, // Attack100Start
        0..4294967295, // Attack100Loop
        0..4294967295, // Attack100End
        7..17, // AttackDash
        9..12, // AttackS3Hi
        9..12, // AttackS3HiS
        9..12, // AttackS3S
        9..12, // AttackS3LwS
        9..12, // AttackS3Lw
        17..22, // AttackHi3
        10..16, // AttackLw3
        18..4294967295, // AttackS4Hi
        0..4294967295, // 
        18..4294967295, // AttackS4S
        0..4294967295, // 
        18..4294967295, // AttackS4Lw
        21..29, // AttackHi4
        19..33, // AttackLw4
        7..30, // AttackAirN
        14..31, // AttackAirF
        10..18, // AttackAirB
        6..13, // AttackAirHi
        16..21, // AttackAirLw
    ],
    // DonkeyKong
    &[
        5..8, // Attack11
        4..11, // Attack12
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        9..21, // AttackDash
        8..12, // AttackS3Hi
        0..4294967295, // 
        8..12, // AttackS3S
        0..4294967295, // 
        8..12, // AttackS3Lw
        6..12, // AttackHi3
        6..10, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        22..24, // AttackS4S
        0..4294967295, // 
        0..4294967295, // 
        14..17, // AttackHi4
        10..14, // AttackLw4
        10..27, // AttackAirN
        25..30, // AttackAirF
        7..16, // AttackAirB
        6..9, // AttackAirHi
        18..24, // AttackAirLw
    ],
    // Kirby
    &[
        3..5, // Attack11
        2..4, // Attack12
        0..4294967295, // 
        0..4294967295, // Attack100Start
        0..4294967295, // Attack100Loop
        0..4294967295, // Attack100End
        9..44, // AttackDash
        0..4294967295, // AttackS3Hi
        0..4294967295, // 
        5..9, // AttackS3S
        0..4294967295, // 
        0..4294967295, // AttackS3Lw
        4..8, // AttackHi3
        4..8, // AttackLw3
        0..4294967295, // AttackS4Hi
        0..4294967295, // 
        13..22, // AttackS4S
        0..4294967295, // 
        0..4294967295, // AttackS4Lw
        13..24, // AttackHi4
        7..23, // AttackLw4
        10..35, // AttackAirN
        10..27, // AttackAirF
        6..21, // AttackAirB
        11..14, // AttackAirHi
        18..40, // AttackAirLw
    ],
    // Bowser
    &[
        7..10, // Attack11
        8..11, // Attack12
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        10..18, // AttackDash
        12..17, // AttackS3Hi
        0..4294967295, // 
        12..17, // AttackS3S
        0..4294967295, // 
        12..17, // AttackS3Lw
        7..11, // AttackHi3
        14..32, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        29..34, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        16..31, // AttackHi4
        14..33, // AttackLw4
        8..24, // AttackAirN
        8..12, // AttackAirF
        9..18, // AttackAirB
        22..26, // AttackAirHi
        14..40, // AttackAirLw
    ],
    // Link
    &[
        6..9, // Attack11
        6..8, // Attack12
        6..11, // Attack13
        0..4294967295, // Attack100Start
        0..4294967295, // Attack100Loop
        0..4294967295, // Attack100End
        7..13, // AttackDash
        0..4294967295, // 
        0..4294967295, // 
        16..20, // AttackS3
        0..4294967295, // 
        0..4294967295, // 
        9..16, // AttackHi3
        14..17, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        15..19, // AttackS41
        0..4294967295, // 
        0..4294967295, // 
        11..44, // AttackHi4
        9..24, // AttackLw4
        4..4294967295, // AttackAirN
        14..34, // AttackAirF
        6..24, // AttackAirB
        5..50, // AttackAirHi
        13..65, // AttackAirLw
    ],
    // Sheik
    &[
        2..4, // Attack11
        2..5, // Attack12
        0..4294967295, // 
        0..4294967295, // Attack100Start
        0..4294967295, // Attack100Loop
        0..4294967295, // Attack100End
        6..13, // AttackDash
        0..4294967295, // 
        0..4294967295, // 
        5..11, // AttackS3
        0..4294967295, // 
        0..4294967295, // 
        5..25, // AttackHi3
        5..9, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        12..30, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        12..17, // AttackHi4
        5..25, // AttackLw4
        3..31, // AttackAirN
        5..8, // AttackAirF
        4..20, // AttackAirB
        5..21, // AttackAirHi
        15..34, // AttackAirLw
    ],
    // Ness
    &[
        3..5, // Attack11
        3..5, // Attack12
        6..10, // Attack13
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        8..23, // AttackDash
        7..12, // AttackS3Hi
        0..4294967295, // AttackS3HiS
        7..12, // AttackS3S
        0..4294967295, // AttackS3LwS
        7..12, // AttackS3Lw
        5..10, // AttackHi3
        3..6, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        16..18, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        12..4294967295, // AttackHi4
        12..4294967295, // AttackLw4
        5..24, // AttackAirN
        8..22, // AttackAirF
        10..20, // AttackAirB
        8..12, // AttackAirHi
        20..29, // AttackAirLw
    ],
    // Peach
    &[
        2..4, // Attack11
        2..4, // Attack12
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        6..21, // AttackDash
        6..14, // 
        0..4294967295, // 
        6..14, // AttackS3
        0..4294967295, // 
        0..4294967295, // 
        9..14, // AttackHi3
        12..14, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        13..23, // AttackHi4
        5..23, // AttackLw4
        3..24, // AttackAirN
        16..21, // AttackAirF
        6..23, // AttackAirB
        7..12, // AttackAirHi
        12..36, // AttackAirLw
    ],
    // Popo
    &[
        4..8, // Attack11
        4..7, // Attack12
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        11..13, // AttackDash
        0..4294967295, // AttackS3Hi
        0..4294967295, // AttackS3HiS
        6..10, // AttackS3S
        0..4294967295, // AttackS3LwS
        0..4294967295, // AttackS3Lw
        8..27, // AttackHi3
        8..12, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        13..15, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        14..19, // AttackHi4
        6..12, // AttackLw4
        6..24, // AttackAirN
        19..23, // AttackAirF
        8..12, // AttackAirB
        6..24, // AttackAirHi
        3..53, // AttackAirLw
    ],
    // Nana
    &[
        4..8, // 
        4..7, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        11..13, // 
        0..4294967295, // 
        0..4294967295, // 
        6..10, // 
        0..4294967295, // 
        0..4294967295, // 
        8..27, // 
        8..12, // 
        0..4294967295, // 
        0..4294967295, // 
        13..15, // 
        0..4294967295, // 
        0..4294967295, // 
        14..19, // 
        6..12, // 
        6..24, // 
        19..23, // 
        8..12, // 
        6..24, // 
        3..53, // 
    ],
    // Pikachu
    &[
        2..4, // Attack11
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        5..17, // AttackDash
        5..15, // AttackS3Hi
        0..4294967295, // 
        5..15, // AttackS3S
        0..4294967295, // 
        5..15, // AttackS3Lw
        7..15, // AttackHi3
        7..10, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        16..24, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        8..18, // AttackHi4
        7..26, // AttackLw4
        3..29, // AttackAirN
        10..25, // AttackAirF
        4..38, // AttackAirB
        3..9, // AttackAirHi
        14..27, // AttackAirLw
    ],
    // Samus
    &[
        3..5, // Attack11
        4..7, // Attack12
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        7..17, // AttackDash
        6..9, // AttackS3Hi
        6..9, // AttackS3HiS
        6..9, // AttackS3S
        6..9, // AttackS3LwS
        6..9, // AttackS3Lw
        14..18, // AttackHi3
        6..9, // AttackLw3
        10..4294967295, // AttackS4Hi
        10..4294967295, // AttackS4HiS
        10..4294967295, // AttackS4S
        10..4294967295, // AttackS4LwS
        10..4294967295, // AttackS4Lw
        12..30, // AttackHi4
        6..17, // AttackLw4
        5..30, // AttackAirN
        5..33, // AttackAirF
        9..13, // AttackAirB
        5..22, // AttackAirHi
        18..23, // AttackAirLw
    ],
    // Yoshi
    &[
        3..6, // Attack11
        3..6, // Attack12
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        10..24, // AttackDash
        6..9, // AttackS3Hi
        0..4294967295, // AttackS3HiS
        6..9, // AttackS3S
        0..4294967295, // AttackS3LwS
        6..9, // AttackS3Lw
        8..13, // AttackHi3
        8..11, // AttackLw3
        0..4294967295, // AttackS4Hi
        0..4294967295, // 
        14..17, // AttackS4S
        0..4294967295, // 
        0..4294967295, // AttackS4Lw
        11..16, // AttackHi4
        6..23, // AttackLw4
        3..34, // AttackAirN
        19..22, // AttackAirF
        10..31, // AttackAirB
        5..7, // AttackAirHi
        18..46, // AttackAirLw
    ],
    // Jigglypuff
    &[
        5..7, // Attack11
        5..7, // Attack12
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        4..15, // AttackDash
        0..4294967295, // AttackS3Hi
        0..4294967295, // 
        6..10, // AttackS3S
        0..4294967295, // 
        0..4294967295, // AttackS3Lw
        8..15, // AttackHi3
        10..13, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        12..21, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        7..11, // AttackHi4
        9..11, // AttackLw4
        6..29, // AttackAirN
        7..23, // AttackAirF
        9..13, // AttackAirB
        9..13, // AttackAirHi
        5..33, // AttackAirLw
    ],
    // Mewtwo
    &[
        8..9, // Attack11
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // Attack100Start
        0..4294967295, // Attack100Loop
        0..4294967295, // Attack100End
        10..30, // AttackDash
        6..9, // AttackS3Hi
        0..4294967295, // 
        6..9, // AttackS3S
        0..4294967295, // 
        6..9, // AttackS3Lw
        6..12, // AttackHi3
        5..17, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        18..20, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        9..35, // AttackHi4
        20..22, // AttackLw4
        5..39, // AttackAirN
        5..8, // AttackAirF
        12..16, // AttackAirB
        9..12, // AttackAirHi
        18..22, // AttackAirLw
    ],
    // Luigi
    &[
        2..4, // Attack11
        2..4, // Attack12
        4..6, // Attack13
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        4..38, // AttackDash
        4..9, // AttackS3Hi
        0..4294967295, // 
        4..9, // AttackS3S
        0..4294967295, // 
        4..9, // AttackS3Lw
        4..13, // AttackHi3
        5..9, // AttackLw3
        12..14, // AttackS4Hi
        0..4294967295, // 
        12..14, // AttackS4S
        0..4294967295, // 
        12..14, // AttackS4Lw
        9..12, // AttackHi4
        5..16, // AttackLw4
        3..32, // AttackAirN
        7..11, // AttackAirF
        6..18, // AttackAirB
        5..8, // AttackAirHi
        10..15, // AttackAirLw
    ],
    // Marth
    &[
        4..8, // Attack11
        4..9, // Attack12
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        12..16, // AttackDash
        0..4294967295, // 
        0..4294967295, // 
        7..11, // AttackS31
        0..4294967295, // 
        0..4294967295, // 
        6..13, // AttackHi3
        7..10, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        10..14, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        13..17, // AttackHi4
        5..23, // AttackLw4
        6..22, // AttackAirN
        4..8, // AttackAirF
        7..12, // AttackAirB
        5..9, // AttackAirHi
        6..10, // AttackAirLw
    ],
    // Zelda
    &[
        11..16, // Attack11
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        6..14, // AttackDash
        0..4294967295, // AttackS3Hi
        0..4294967295, // AttackS3HiS
        12..15, // AttackS3S
        0..4294967295, // AttackS3LwS
        0..4294967295, // AttackS3Lw
        10..25, // AttackHi3
        5..8, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        16..25, // AttackS4S
        0..4294967295, // 
        0..4294967295, // 
        5..35, // AttackHi4
        4..17, // AttackLw4
        6..28, // AttackAirN
        8..12, // AttackAirF
        5..9, // AttackAirB
        14..17, // AttackAirHi
        14..18, // AttackAirLw
    ],
    // YoungLink
    &[
        6..9, // Attack11
        6..8, // Attack12
        6..11, // Attack13
        0..4294967295, // Attack100Start
        0..4294967295, // Attack100Loop
        0..4294967295, // Attack100End
        7..13, // AttackDash
        0..4294967295, // 
        0..4294967295, // 
        11..14, // AttackS3
        0..4294967295, // 
        0..4294967295, // 
        9..16, // AttackHi3
        14..17, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        15..18, // AttackS41
        0..4294967295, // 
        0..4294967295, // 
        11..45, // AttackHi4
        9..24, // AttackLw4
        4..28, // AttackAirN
        14..34, // AttackAirF
        6..24, // AttackAirB
        5..50, // AttackAirHi
        13..65, // AttackAirLw
    ],
    // DrMario
    &[
        2..4, // Attack11
        2..4, // Attack12
        4..9, // Attack13
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        6..26, // AttackDash
        4..9, // AttackS3Hi
        0..4294967295, // 
        4..9, // AttackS3S
        0..4294967295, // 
        4..9, // AttackS3Lw
        4..14, // AttackHi3
        5..9, // AttackLw3
        12..17, // AttackS4Hi
        0..4294967295, // 
        12..17, // AttackS4S
        0..4294967295, // 
        12..17, // AttackS4Lw
        9..12, // AttackHi4
        5..16, // AttackLw4
        3..32, // AttackAirN
        18..23, // AttackAirF
        6..17, // AttackAirB
        4..10, // AttackAirHi
        10..30, // AttackAirLw
    ],
    // Falco
    &[
        2..4, // Attack11
        2..4, // Attack12
        0..4294967295, // 
        0..4294967295, // Attack100Start
        0..4294967295, // Attack100Loop
        0..4294967295, // Attack100End
        4..18, // AttackDash
        0..4294967295, // AttackS3Hi
        0..4294967295, // AttackS3HiS
        5..10, // AttackS3S
        0..4294967295, // AttackS3LwS
        0..4294967295, // AttackS3Lw
        5..12, // AttackHi3
        7..10, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        12..22, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        7..16, // AttackHi4
        6..11, // AttackLw4
        4..32, // AttackAirN
        6..46, // AttackAirF
        4..20, // AttackAirB
        8..15, // AttackAirHi
        5..25, // AttackAirLw
    ],
    // Pichu
    &[
        2..4, // Attack11
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        5..17, // AttackDash
        5..15, // AttackS3Hi
        0..4294967295, // 
        5..15, // AttackS3S
        0..4294967295, // 
        5..15, // AttackS3Lw
        7..15, // AttackHi3
        7..10, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        16..35, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        9..12, // AttackHi4
        7..14, // AttackLw4
        3..29, // AttackAirN
        10..25, // AttackAirF
        4..38, // AttackAirB
        4..10, // AttackAirHi
        14..27, // AttackAirLw
    ],
    // GameAndWatch
    &[
        4..7, // Attack11
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // Attack100Start
        0..4294967295, // Attack100Loop
        0..4294967295, // Attack100End
        6..30, // AttackDash
        0..4294967295, // 
        0..4294967295, // 
        13..31, // AttackS3
        0..4294967295, // 
        0..4294967295, // 
        9..30, // AttackHi3
        6..14, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        13..34, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        24..29, // AttackHi4
        15..20, // AttackLw4
        20..30, // AttackAirN
        10..33, // AttackAirF
        10..22, // AttackAirB
        7..23, // AttackAirHi
        12..39, // AttackAirLw
    ],
    // Ganondorf
    &[
        3..6, // Attack11
        0..4294967295, // Attack12
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        7..17, // AttackDash
        9..12, // AttackS3Hi
        9..12, // AttackS3HiS
        9..12, // AttackS3S
        9..12, // AttackS3LwS
        9..12, // AttackS3Lw
        81..84, // AttackHi3
        10..13, // AttackLw3
        20..4294967295, // AttackS4Hi
        0..4294967295, // 
        20..4294967295, // AttackS4S
        0..4294967295, // 
        20..4294967295, // AttackS4Lw
        21..30, // AttackHi4
        19..33, // AttackLw4
        7..22, // AttackAirN
        14..20, // AttackAirF
        10..16, // AttackAirB
        6..17, // AttackAirHi
        16..21, // AttackAirLw
    ],
    // Roy
    &[
        4..8, // Attack11
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        0..4294967295, // 
        12..16, // AttackDash
        0..4294967295, // 
        0..4294967295, // 
        9..14, // AttackS31
        0..4294967295, // 
        0..4294967295, // 
        7..14, // AttackHi3
        8..11, // AttackLw3
        0..4294967295, // 
        0..4294967295, // 
        12..15, // AttackS4
        0..4294967295, // 
        0..4294967295, // 
        15..25, // AttackHi4
        6..26, // AttackLw4
        7..21, // AttackAirN
        5..8, // AttackAirF
        8..11, // AttackAirB
        5..11, // AttackAirHi
        7..11, // AttackAirLw
    ],
];
