# Training Mode Replay Writer

Writes the gci savestate files read by UnclePunch's Training Mode.

## Format

There are a few layers in play here:
1. gci file format
2. melee memory blocks and encoding
3. another melee block format
4. UnclePunch's savestate and replay formats

Note that everything is in big endian format.

### GCI File format

The outermost layer. 
Documentation on this format is prevalent,
such as [https://web.archive.org/web/20040520074333/http://members.iinet.net.au/~theimp/gci/GameCube%20GCI%20&%20GCP%20Memory%20Card%20Save%20File%20Format%20Specifications.pdf][here].
The block format data for unclepunch replays seem to start at offset 0x1EB0 in the file until the end of the file.
None of the header data seems to be relevant here except for the block count (offset 0x38-0x40). 
The rest of the header can be copied from an example gci file.
There are no icon data stored, the screenshot used in unclepunch is stored elsewhere.

### Melee outer block format

Huge thank you to Cuyler36, Altafen, and Reno in the GameCube decompilation discord for their help in decompilation.
Without them, reverse engineering would have taken 1000x longer.

The raw inner data are separated into blocks of maximum size 8192. 
The first block always seems to 400 bytes and the rest are all 8192 bytes. 
The last block is padded to 8192 bytes.

Each block starts with a 16 bytes checksum, with the rest being encrypted inner data.
There is another checksum at 0x1E80 covering the data from 0x40 to that point.

The code to encrypt and decrypt these blocks can be found in `obfuscation.c`.

### Melee inner block format

Each decrypted block from the outer format starts with 16 bytes of metadata.

```
block idx       total len
    ^              ^
  |   |          |   |
  -- -- 00 00 c0 -- -- 01 00 00 00 02 00 00 00 00 
```

- The first 2 bytes is the block index. This increments for each block.
- The 6th and 7th bytes is the total len. This is the same for each block.
I don't know what the other 12 bytes do, but they're the same for each block in every training mode replay file.

Concatenate the rest of the data in the block (having removed the checksum and metadata) to recover the raw inner data.

### UnclePunch's format

This can be found in the [https://github.com/UnclePunch/Training-Mode/](repo) under `patch/events/lab/source/lab.c` and `patch/events/lab/source/lab.h`.

1. There is a header `ExportHeader` with some misc data (stage, characters, date, offsets, etc.).
2. That is followed by the screenshot, which is a 96x72 RGB565 encoded image. This is always 0x3600 bytes.
3. Then comes the `RecordingSave` struct. This is lz77 compressed. It contains the raw savestate, event data and the inputs.
4. Finally, added in v2 of the replay file format, comes the menu data `ExportMenuSettings`.
