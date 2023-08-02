# unencrypted
16 // PTLS_CONTENT_TYPE_HANDSHAKE
03 03 00 00

14 // PTLS_HANDSHAKE_TYPE_FINISHED
00 00 30 // len


// verify data
66 5a 14 d6 3b f3 f6 4b 30 b0
0f 9c 12 a6 3a 2d 47 b6 d5 ed
70 2d 8c ff f8 b4 96 3a 3c 77
26 fd 3d da 63 b2 13 b6 20 4d
1b 80 e0 1f bc f2 34 7a

# encrypted
17 // PTLS_CONTENT_TYPE_APPDATA
03 03 00 45

fb // PTLS_HANDSHAKE_TYPE_FINISHED
01 f2 1a // len

// verify data
db 23 16 ec ec a9 5f e9 71 b2
89 cb 0a d5 9e 9d a1 8f bc 8d
68 b0 52 50 8c 23 46 84 51 ca
8c ad 50 73 17 6d 51 ec 87 85
95 e9 24 c6 92 d2 47 42

df // PTLS_CONTENT_TYPE_HANDSHAKE

// tag
a5 96 7f 31 d6 a0 76 61 97 ea
e8 fb 4d dd 19 57