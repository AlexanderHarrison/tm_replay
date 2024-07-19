#include "obfuscation.h"

static const int ENCODE_LUT[13] = {
    0x26, 0xFF, 0xE8, 0xEF, 0x42, 0xD6, 0x01, 0x54, 0x14, 0xA3, 0x80, 0xFD, 0x6E
};

// DECODE ===============================================================

// https://decomp.me/scratch/qqiBz
// 803b302c: (MemoryCard_Deobfuscate) in GTME01.map
// decoded by Cuyler36
static int deobfuscate_byte(int r3, int r4) {
    int b = r3 & 0xFF;
    int r5;

       switch (b % 7) {
        case 0:
            r5 = (int)((r4 & 0x01) << 0) |
                 (int)((r4 & 0x02) << 1) |
                 (int)((r4 & 0x04) << 2) |
                 (int)((r4 & 0x08) << 3) |
                 (int)((r4 & 0x10) >> 3) |
                 (int)((r4 & 0x20) >> 2) |
                 (int)((r4 & 0x40) >> 1) |
                 (int)((r4 & 0x80) >> 0);
            r4 = r5 & 0xFF;
            break;
        case 1:
            r5 = (int)((r4 & 0x01) << 1) |
                 (int)((r4 & 0x02) << 6) |
                 (int)((r4 & 0x04) << 0) |
                 (int)((r4 & 0x08) >> 3) |
                 (int)((r4 & 0x10) << 1) |
                 (int)((r4 & 0x20) >> 1) |
                 (int)((r4 & 0x40) >> 3) |
                 (int)((r4 & 0x80) >> 1);
            r4 = r5 & 0xFF;
            break;
        case 2:
            r5 = (int)((r4 & 0x01) << 2) |
                 (int)((r4 & 0x02) << 2) |
                 (int)((r4 & 0x04) << 4) |
                 (int)((r4 & 0x08) << 1) |
                 (int)((r4 & 0x10) << 3) |
                 (int)((r4 & 0x20) >> 4) |
                 (int)((r4 & 0x40) >> 6) |
                 (int)((r4 & 0x80) >> 2);
            r4 = r5 & 0xFF;
            break;
        case 3:
            r5 = (int)((r4 & 0x01) << 4) |
                 (int)((r4 & 0x02) >> 1) |
                 (int)((r4 & 0x04) << 3) |
                 (int)((r4 & 0x08) >> 2) |
                 (int)((r4 & 0x10) >> 1) |
                 (int)((r4 & 0x20) << 1) |
                 (int)((r4 & 0x40) << 1) |
                 (int)((r4 & 0x80) >> 5);
            r4 = r5 & 0xFF;
            break;
        case 4:
            r5 = (int)((r4 & 0x01) << 3) |
                 (int)((r4 & 0x02) << 4) |
                 (int)((r4 & 0x04) >> 1) |
                 (int)((r4 & 0x08) << 4) |
                 (int)((r4 & 0x10) << 2) |
                 (int)((r4 & 0x20) >> 3) |
                 (int)((r4 & 0x40) >> 2) |
                 (int)((r4 & 0x80) >> 7);
            r4 = r5 & 0xFF;
            break;
        case 5:
            r5 = (int)((r4 & 0x01) << 5) |
                 (int)((r4 & 0x02) << 5) |
                 (int)((r4 & 0x04) << 5) |
                 (int)((r4 & 0x08) >> 0) |
                 (int)((r4 & 0x10) >> 2) |
                 (int)((r4 & 0x20) >> 5) |
                 (int)((r4 & 0x40) >> 5) |
                 (int)((r4 & 0x80) >> 3);
            r4 = r5 & 0xFF;
            break;
        case 6:
            r5 = (int)((r4 & 0x01) << 6) |
                 (int)((r4 & 0x02) << 0) |
                 (int)((r4 & 0x04) >> 2) |
                 (int)((r4 & 0x08) << 2) |
                 (int)((r4 & 0x10) << 0) |
                 (int)((r4 & 0x20) << 2) |
                 (int)((r4 & 0x40) >> 4) |
                 (int)((r4 & 0x80) >> 4);
            r4 = r5 & 0xFF;
            break;
    }

    r4 ^= ENCODE_LUT[b % 13];
    r4 ^= r3;
    return r4;
}


// https://decomp.me/scratch/AQnIM
// 0x803B2B20: (MemoryCard_CalcChecksum) unnamed in GTME01.map
// decoded by Altafen and Aitch
void calculate_checksum(U8 *src, int len, U8 *result) {
    U8 checksum[16];
    int i;

    checksum[0] = 0x01;
    checksum[1] = 0x23;
    checksum[2] = 0x45;
    checksum[3] = 0x67;
    checksum[4] = 0x89;
    checksum[5] = 0xAB;
    checksum[6] = 0xCD;
    checksum[7] = 0xEF;
    checksum[8] = 0xFE;
    checksum[9] = 0xDC;
    checksum[10] = 0xBA;
    checksum[11] = 0x98;
    checksum[12] = 0x76;
    checksum[13] = 0x54;
    checksum[14] = 0x32;
    checksum[15] = 0x10;

        
    for (i = 0; i < len; i++) {
        checksum[i % 16] += src[0];
        src += 1;
    }
        
    for (i = 1; i < 16; i++) {
        if (checksum[i-1] == checksum[i]) {
            checksum[i] ^= 0xFF;
        }
    }

    memcpy(result, checksum, 16);
}


// https://decomp.me/scratch/Z68g0
// 0x803B31CC: (MemoryCard_DecodeMain) in GTME01.map
// decoded by Altafen
int decode_block(U8* src, int len) {
    int i;
    int x, y;
    U8 checksum[16];
    if (src == NULL) {
        return 0xFFFFFFFF;
    }
    x = src[15];
    for (i = 16; i < len; i++) {
        y = src[i];
        src[i] = deobfuscate_byte(x, y);
        x = y;
    }
    calculate_checksum(src + 16, len - 16, checksum);

    int no_match = 0;
    for (i = 0; i < 16; i++) {
        if (src[i] != checksum[i]) {
            no_match += 1;
        }
    }
        
    return no_match;
}

// ENCODE ===================================================================

// https://decomp.me/scratch/eBWr6
// 0x803B2E04: (MemoryCard_Deobfuscate) in GTME01.map
// decoded by Cuyler36 and Aitch
static U8 obfuscate_byte(U8 prev, U8 this) {
    int r3 = prev;
    int r4 = this;

    int r5;
    int b = r3 & 0xFF;
    r4 = b ^ r4;
    r4 ^= ENCODE_LUT[b % 13];

   switch (b % 7) {
        case 0:
            r5 = (int)((r4 & 0x01) << 0) |
                 (int)((r4 & 0x02) << 3) |
                 (int)((r4 & 0x04) >> 1) |
                 (int)((r4 & 0x08) << 2) |
                 (int)((r4 & 0x10) >> 2) |
                 (int)((r4 & 0x20) << 1) |
                 (int)((r4 & 0x40) >> 3) |
                 (int)((r4 & 0x80) >> 0);
            r4 = r5 & 0xFF;
            return r4;
        case 1:
            r5 = (int)((r4 & 0x01) << 3) |
                 (int)((r4 & 0x02) >> 1) |
                 (int)((r4 & 0x04) << 0) |
                 (int)((r4 & 0x08) << 3) |
                 (int)((r4 & 0x10) << 1) |
                 (int)((r4 & 0x20) >> 1) |
                 (int)((r4 & 0x40) << 1) |
                 (int)((r4 & 0x80) >> 6);
            r4 = r5 & 0xFF;
            return r4;
        case 2:
            r5 = (int)((r4 & 0x01) << 6) |
                 (int)((r4 & 0x02) << 4) |
                 (int)((r4 & 0x04) >> 2) |
                 (int)((r4 & 0x08) >> 2) |
                 (int)((r4 & 0x10) >> 1) |
                 (int)((r4 & 0x20) << 2) |
                 (int)((r4 & 0x40) >> 4) |
                 (int)((r4 & 0x80) >> 3);
            r4 = r5 & 0xFF;
            return r4;
        case 3:
            r5 = (int)((r4 & 0x01) << 1) |
                 (int)((r4 & 0x02) << 2) |
                 (int)((r4 & 0x04) << 5) |
                 (int)((r4 & 0x08) << 1) |
                 (int)((r4 & 0x10) >> 4) |
                 (int)((r4 & 0x20) >> 3) |
                 (int)((r4 & 0x40) >> 1) |
                 (int)((r4 & 0x80) >> 1);
            r4 = r5 & 0xFF;
            return r4;
        case 4:
            r5 = (int)((r4 & 0x01) << 7) |
                 (int)((r4 & 0x02) << 1) |
                 (int)((r4 & 0x04) << 3) |
                 (int)((r4 & 0x08) >> 3) |
                 (int)((r4 & 0x10) << 2) |
                 (int)((r4 & 0x20) >> 4) |
                 (int)((r4 & 0x40) >> 2) |
                 (int)((r4 & 0x80) >> 4);
            r4 = r5 & 0xFF;
            return r4;
        case 5:
            r5 = (int)((r4 & 0x01) << 5) |
                 (int)((r4 & 0x02) << 5) |
                 (int)((r4 & 0x04) << 2) |
                 (int)((r4 & 0x08) >> 0) |
                 (int)((r4 & 0x10) << 3) |
                 (int)((r4 & 0x20) >> 5) |
                 (int)((r4 & 0x40) >> 5) |
                 (int)((r4 & 0x80) >> 5);
            r4 = r5 & 0xFF;
            return r4;
        case 6:
            r5 = (int)((r4 & 0x01) << 2) |
                 (int)((r4 & 0x02) << 0) |
                 (int)((r4 & 0x04) << 4) |
                 (int)((r4 & 0x08) << 4) |
                 (int)((r4 & 0x10) << 0) |
                 (int)((r4 & 0x20) >> 2) |
                 (int)((r4 & 0x40) >> 6) |
                 (int)((r4 & 0x80) >> 2);
            r4 = r5 & 0xFF;
            return r4;
    }
    
    return r4;
}

// https://decomp.me/scratch/uJB7I
// 0x803B2FA0: (MemoryCard_EncodeBlock) unnamed in GTME01.map
// decoded by Aitch
int encode_block(U8 *data, size_t length) {
    size_t i;
    
    if (data == NULL) return -1;

    calculate_checksum(data + 0x10, length - 0x10, data);

    for (i = 0x10; (int)i < (int)length; ++i) {
        data[i] = obfuscate_byte(data[i-1], data[i]);
    }
    
    return 0;
}
