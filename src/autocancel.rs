pub struct LagWindows {
    pub nair: std::ops::Range<u32>,
    pub uair: std::ops::Range<u32>,
    pub bair: std::ops::Range<u32>,
    pub fair: std::ops::Range<u32>,
    pub dair: std::ops::Range<u32>,
}

// TODO Increment last value
pub const AERIAL_LAG_WINDOWS: [LagWindows; 27] = [
    LagWindows { // Mario
        nair: 3..36,
        fair: 3..43,
        bair: 6..19,
        uair: 2..16,
        dair: 6..36,
    },
    LagWindows { // Fox
        nair: 4..37,
        fair: 6..49,
        bair: 4..23,
        uair: 8..26,
        dair: 5..34,
    },
    LagWindows { // Captain Falcon
        nair: 4..34,
        fair: 7..35,
        bair: 7..21,
        uair: 1..22,
        dair: 4..36,
    },
    LagWindows { // Donkey Kong
        nair: 10..39,
        fair: 1..60,
        bair: 7..20,
        uair: 6..13,
        dair: 3..50,
    },
    LagWindows { // Kirby
        nair: 10..38,
        fair: 10..38,
        bair: 6..28,
        uair: 11..17,
        dair: 18..48,
    },
    LagWindows { // Bowser
        nair: 8..40,
        fair: 9..31,
        bair: 9..45,
        uair: 10..40,
        dair: 14..70,
    },
    LagWindows { // Link
        nair: 4..32,
        fair: 1..51,
        bair: 1..29,
        uair: 5..56,
        dair: 13..65,
    },
    LagWindows { // Sheik
        nair: 3..31,
        fair: 5..11,
        bair: 4..25,
        uair: 5..30,
        dair: 3..49,
    },
    LagWindows { // Ness
        nair: 5..27,
        fair: 8..30,
        bair: 10..25,
        uair: 8..27,
        dair: 20..29,
    },
    LagWindows { // Peach
        nair: 3..36,
        fair: 16..39,
        bair: 6..23,
        uair: 7..22,
        dair: 12..40,
    },
    LagWindows { // Ice Climbers - Popo
        nair: 6..30,
        fair: 3..24,
        bair: 8..19,
        uair: 6..27,
        dair: 3..58,
    },
    LagWindows { // Ice Climbers - Nana
        nair: 6..30,
        fair: 3..24,
        bair: 8..19,
        uair: 6..27,
        dair: 3..58,
    },
    LagWindows { // Pikachu
        nair: 3..35,
        fair: 10..38,
        bair: 4..50,
        uair: 3..18,
        dair: 1..39,
    },
    LagWindows { // Samus
        nair: 5..35,
        fair: 1..47,
        bair: 9..31,
        uair: 5..34,
        dair: 3..34,
    },
    LagWindows { // Yoshi
        nair: 3..36,
        fair: 4..36,
        bair: 10..33,
        uair: 5..33,
        dair: 16..60,
    },
    LagWindows { // Jigglypuff
        nair: 6..29,
        fair: 7..34,
        bair: 9..25,
        uair: 9..37,
        dair: 5..42,
    },
    LagWindows { // Mewtwo
        nair: 5..44,
        fair: 1..35,
        bair: 3..30,
        uair: 4..33,
        dair: 6..45,
    },
    LagWindows { // Luigi
        nair: 3..36,
        fair: 2..20,
        bair: 6..19,
        uair: 2..16,
        dair: 6..24,
    },
    LagWindows { // Marth
        nair: 6..25,
        fair: 1..27,
        bair: 1..32,
        uair: 5..27,
        dair: 6..48,
    },
    LagWindows { // Zelda
        nair: 6..38,
        fair: 8..25,
        bair: 5..26,
        uair: 14..45,
        dair: 1..40,
    },
    LagWindows { // Young Link
        nair: 4..32,
        fair: 1..47,
        bair: 1..29,
        uair: 5..56,
        dair: 13..65,
    },
    LagWindows { // Dr Mario
        nair: 3..36,
        fair: 3..43,
        bair: 6..19,
        uair: 2..16,
        dair: 6..36,
    },
    LagWindows { // Falco
        nair: 4..37,
        fair: 6..49,
        bair: 4..23,
        uair: 8..26,
        dair: 5..30,
    },
    LagWindows { // Pichu
        nair: 3..35,
        fair: 10..38,
        bair: 4..50,
        uair: 4..18,
        dair: 1..39,
    },
    LagWindows { // Game & Watch
        nair: 3..45,
        fair: 3..45,
        bair: 10..40,
        uair: 7..40,
        dair: 6..50,
    },
    LagWindows { // Ganondorf
        nair: 4..26,
        fair: 7..34,
        bair: 7..19,
        uair: 1..22,
        dair: 4..36,
    },
    LagWindows { // Roy
        nair: 7..32,
        fair: 1..30,
        bair: 1..34,
        uair: 5..30,
        dair: 7..55,
    },
];


// GENERATION:
// I used the dumped json files from here https://github.com/pfirsich/meleeFrameDataExtractor
// And used the following python script to generate the table:
//
//import os
//import json
//
//    c = [
//        "Mario",
//        "Fox",
//        "Captain Falcon",
//        "Donkey Kong",
//        "Kirby",
//        "Bowser",
//        "Link",
//        "Sheik",
//        "Ness",
//        "Peach",
//        "Ice Climbers - Popo",
//        "Ice Climbers - Nana",
//        "Pikachu",
//        "Samus",
//        "Yoshi",
//        "Jigglypuff",
//        "Mewtwo",
//        "Luigi",
//        "Marth",
//        "Zelda",
//        "Young Link",
//        "Dr Mario",
//        "Falco",
//        "Pichu",
//        "Game & Watch",
//        "Ganondorf",
//        "Roy",
//    ]
//
//    for ch in c:
//        filepath = ch + ".framedata.json"
//
//        try:
//            with open(filepath, 'r', encoding='utf-8') as file:
//                data = json.load(file)
//                print("LagWindows { // " + ch)
//                for n in ["nair", "fair", "bair", "uair", "dair"]:
//                    print('    ' + n + ": " + str(data[n]["autoCancelBefore"]) + ".." + str(data[n]["autoCancelAfter"] + 1) + ",")
//                print("},")
//                #if value is not None:
//                #    print(f"{filename}: {value}")
//                #else:
//                #    print(f"{filename}: Key 'a.b.c' not found.")
//        except Exception:
//            pass

