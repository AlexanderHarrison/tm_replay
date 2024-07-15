// lz77 functions credited to https://github.com/andyherbert/lz1
// converts to big endian
pub fn lz77_compress(uncompressed_text: &[u8], uncompressed_size: u32, compressed_text: &mut [u8]) -> u32 {
    let pointer_length_width = 8u32;

    let pointer_pos_max = 1 << (16 - pointer_length_width);
    let pointer_length_max = 1 << pointer_length_width;

    compressed_text[0..4].copy_from_slice(&uncompressed_size.to_be_bytes());
    compressed_text[5] = pointer_length_width as u8;
    let mut compressed_pointer = 5;
    let mut output_size = 5;

    let mut coding_pos = 0u32;
    while coding_pos < uncompressed_size {
        let mut pointer_pos = 0;
        let mut pointer_length = 0;

        let mut temp_pointer_pos = 1u32;
        while (temp_pointer_pos < pointer_pos_max) && (temp_pointer_pos <= coding_pos) {
            let mut look_behind = coding_pos - temp_pointer_pos;
            let mut look_ahead = coding_pos;

            let mut temp_pointer_length = 0;
            while {
                let cond = uncompressed_text[look_ahead as usize] == uncompressed_text[look_behind as usize];
                look_ahead += 1;
                look_behind += 1;

                cond
            } {
                if temp_pointer_length == pointer_length_max { break }

                temp_pointer_length += 1;
            }

            if temp_pointer_length > pointer_length {
                pointer_pos = temp_pointer_pos;
                pointer_length = temp_pointer_length;
                if pointer_length == pointer_length_max { break; }
            }

            temp_pointer_pos += 1;
        }

        coding_pos += pointer_length;
        let output_lookahead_ref;
        let output_pointer: u16;
        if coding_pos == uncompressed_size && pointer_length != 0 {
            output_pointer = if pointer_length == 1 { 
                0 
            } else { 
                ((pointer_pos as u16) << pointer_length_width) | (pointer_length as u16 - 2)
            };
            output_lookahead_ref = coding_pos - 1;
        }
        else
        {
            output_pointer = ((pointer_pos as u16) << pointer_length_width) | if pointer_length != 0 { pointer_length as u16 - 1 } else { 0 };
            output_lookahead_ref = coding_pos;
        }

        compressed_text[compressed_pointer as usize..compressed_pointer as usize+2]
            .copy_from_slice(&output_pointer.to_be_bytes());
        compressed_pointer += 2;
        compressed_text[compressed_pointer as usize] = uncompressed_text[output_lookahead_ref as usize];
        compressed_pointer += 1;
        output_size += 3;
        coding_pos += 1;
    }

    return output_size;
}

// converts from big endian
pub fn _lz77_decompress(compressed_text: &[u8], uncompressed_text: &mut [u8]) -> usize {
    let uncompressed_size = u32::from_be_bytes(compressed_text[0..4].try_into().unwrap()) as usize;
    let pointer_length_width = compressed_text[4];

    let mut compressed_pointer = 5;
    let pointer_length_mask = (2 << pointer_length_width) - 1;

    let mut coding_pos = 0usize;
    while coding_pos < uncompressed_size {
        let input_pointer = u16::from_be_bytes(compressed_text[compressed_pointer..compressed_pointer+2].try_into().unwrap());
        compressed_pointer += 2;
        let pointer_pos = input_pointer >> pointer_length_width;
        let mut pointer_length = if pointer_pos != 0 { (input_pointer & pointer_length_mask) + 1 } else { 0 };
        if pointer_pos != 0 {
            let mut pointer_offset = coding_pos;
            while pointer_length > 0 {
                uncompressed_text[coding_pos] = uncompressed_text[pointer_offset];
                coding_pos += 1;
                pointer_offset += 1;
                
                pointer_length -= 1;
            }
        }
        uncompressed_text[coding_pos] = compressed_text[compressed_pointer];
        compressed_pointer += 1;

        coding_pos += 1;
    }

    return coding_pos;
}
