# HexAnalyser

Enter a hex value eg: 0xc1bc7ae1 and you will recieve decoded value in Big and Little endian, Signed and Unsigned, Floats, colour and Emoji.

## Installation

To install the project, run:
```
cargo install hex_analyser
```

## Usage

To run the project, execute the installed binary with a hex value:
```
hex_analyser -v 0xc1bc7ae1
```

**Example Output:**
```
❯ hex_analyser --hexval 0xc1bc7ae1
-----------Big Endian-----------
Hex: c1bc7ae1
Unsigned: 3250354913
Signed: 3250354913
Float: -23.56

-----------Little Endian-----------
Hex: e17abcc1
Unsigned: 3782917313
Signed: -512049983
Float: -289080450000000000000

-----------Or it could be-----------
Binary: 11000001101111000111101011100001
Octal: 30157075341
ASCII: ..z. (non-printable characters are replaced with a dot)
```

