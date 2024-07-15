# Training Mode Replay Writer

Writes the gci savestate files read by UnclePunch's Training Mode.

## Format

There are a few layers in play here:
1. gci file format
2. melee memory blocks and encoding
3. another melee block format
4. UnclePunch's savestate and replay formats

### GCI File format

The outermost layer. 
Documentation on this format is prevalent,
such as [https://web.archive.org/web/20040520074333/http://members.iinet.net.au/~theimp/gci/GameCube%20GCI%20&%20GCP%20Memory%20Card%20Save%20File%20Format%20Specifications.pdf][here].
The block format data for unclepunch replays seem to start at offset 0x1EB0 in the file until the end of the file.
None of the header data seems to be relevant here, and can be copied from an example gci file.
There are no icon data stored, the screenshot used in unclepunch is stored elsewhere.

### Melee outer block format

This file format was previously undocumented.
Thanks to Cuyler36, Altafen, and Reno in the GameCube decompilation discord for their help in decompilation.

The raw inner data are separated into blocks of maximum size 8192. 
The first block always seems to 400 bytes and the rest are all 8192 bytes. 
The last block is padded to 8192 bytes.

Each block starts with a 16 bytes checksum, with the rest being encrypted inner data.

The code to encrypt and decrypt these blocks can be found in `obfuscation.c`.

### Melee inner block format

Each decrypted block in the outer format starts with 16 bytes of metadata.
The first 4 bytes is the block index.
I don't know what the next 12 bytes is, but it's the same for every decrypted block.

Concatenate the rest of the data in the blocks to recover the raw inner data.

### UnclePunch's format
