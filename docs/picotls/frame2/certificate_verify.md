# unencrypted
16 // PTLS_CONTENT_TYPE_HANDSHAKE
03 03 00 00

0f // PTLS_HANDSHAKE_TYPE_CERTIFICATE_VERIFY
00 00 4a

04 03

00 46
30 44 02 20 29 91 6d 0b bf 19
4c 93 35 5f 63 22 46 2c 3d 30
25 cc e0 a6 07 49 61 40 21 3f
10 63 9a a4 dd 83 02 20 2a 71
d5 4f d3 10 f7 67 91 75 7b 3d
23 d0 0b 4d 0f 58 ac d3 0b b1
db 72 cb 36 f8 58 98 a2 c7 ae

# encrypted
17 // PTLS_CONTENT_TYPE_APPDATA
03 03 00 5f

9a // encrypted: PTLS_HANDSHAKE_TYPE_CERTIFICATE_VERIFY
03 cb bc

11 6e

e7 18
98 58 fd 3c 0e 2f 18 ef 6a e3
7e 85 67 d8 0f e0 1b 08 03 bc
36 65 43 c4 21 1a 68 62 86 f9
9a 09 d9 32 d1 e2 02 dc bf cc
83 4f 48 cb 02 db 1b 8d c4 d1
29 1d c5 6a 68 3a 1d a3 4b e3
b2 e6 cd 9a 18 ab cb 04 79 e7

80 // encrypted: PTLS_CONTENT_TYPE_HANDSHAKE

// tag
32 c4 4c 87 50 09 c3 4a 12 88
25 ed 3b 79 9f d8