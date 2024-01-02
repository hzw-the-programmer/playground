17 // PTLS_CONTENT_TYPE_APPDATA
03 03 // PTLS_PROTOCOL_VERSION_TLS12
00 35 // len

# encrypted message

61 // PTLS_HANDSHAKE_TYPE_FINISHED
3a 6d 1b // len

// verify_data
9f 45 4e 99 29 a9 23 ce 86 40
36 22 d4 94 aa 11 0d 18 43 f8
fa 5e 31 cf ce be 3d 03 44 49
a9 51 

db // PTLS_CONTENT_TYPE_HANDSHAKE

// tag
ef ec 45 e6 b7 e8 c7 29 2f d0
b9 98 ed 56 a4 ef

# decrypted message

14 // PTLS_HANDSHAKE_TYPE_FINISHED
00 00 20 // len

// verify_data
c5 c4 f4 63 5f e6 4c 22 fa c5
90 89 4c b6 7f 19 f2 72 a7 71
41 ba b2 d8 35 4e 83 1b 96 bb
45 15

16 // PTLS_CONTENT_TYPE_HANDSHAKE