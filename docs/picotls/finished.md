# finished begin
16 // PTLS_CONTENT_TYPE_HANDSHAKE
03 // PTLS_RECORD_VERSION_MAJOR
03 // PTLS_RECORD_VERSION_MINOR
00 00 // len

14 // PTLS_HANDSHAKE_TYPE_FINISHED
00 00 30 // len
d2 bb 82 ec 34 ba 9d 5a 0f a2
d5 03 4b 9b 84 58 b8 0b 31 ae
b2 94 0e 33 30 d5 d8 d1 56 49
17 0f fc 19 d2 1e 98 f9 93 ca
d6 0c a2 35 6a f4 94 1f
# finished end



# encrypted finished begin
17 // PTLS_CONTENT_TYPE_APPDATA
03 // PTLS_RECORD_VERSION_MAJOR
03 // PTLS_RECORD_VERSION_MINOR
00 45 // len

## encrypted content begin
c4 43 76 56 dc 3a 72 6d 10 56
63 f5 b3 7a 3b b2 5d 66 31 5e
9b d2 3d a8 8e 57 75 d1 71 c4
b7 57 2a aa 81 e2 6f 47 83 51
6a 3b 10 52 77 60 13 af b1 0b
4a 88
## encrypted content end

## encrypted content type: PTLS_CONTENT_TYPE_HANDSHAKE
bf

## tag
89 b5 d3 e2 40 f6 ce 69 fc 48
55 c7 1a 84 c0 09
# encrypted finished end