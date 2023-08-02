# unencrypted

## record begin

### record header
16 // PTLS_CONTENT_TYPE_HANDSHAKE
03 03 00 00

## message begin

### message header
14 // PTLS_HANDSHAKE_TYPE_FINISHED
00 00 30 // len

ae d1 77 e1 4d e5 5e e1 21 39
f9 2e 42 21 9f a8 59 8f d7 ce
e6 e6 10 33 36 d6 9c d2 01 ea
a6 62 9b ce 94 dd dd c0 98 1a
0e 8b 56 e3 32 fa 3a a1

## message end

## record end

# encrypted
17 // PTLS_CONTENT_TYPE_APPDATA
03 03 00 45

## message begin
3f // encrypted: PTLS_HANDSHAKE_TYPE_FINISHED
6c 52 8b

a7 02 0d b0 17 eb 27 5f 66 fa
f8 17 1f 21 b3 df 28 86 63 cf
81 87 8c c3 f3 66 8f 4b 0c 76
ff de 34 96 be 7b bf 25 44 d4
9e 30 51 16 e4 92 c0 07
## message end

// encrypted: PTLS_CONTENT_TYPE_HANDSHAKE
8c

// tag
f0 23 5f 8b 73 3e d1 73 11 11
c2 0a c8 1a de 67