
16 // PTLS_CONTENT_TYPE_HANDSHAKE
03 03 // PTLS_PROTOCOL_VERSION_TLS12
00 00 // len

08 // PTLS_HANDSHAKE_TYPE_ENCRYPTED_EXTENSIONS
00 00 37 // len

// extensions
00 35 // len

// PTLS_EXTENSION_TYPE_SERVER_NAME
00 00
00 00

// PTLS_EXTENSION_TYPE_ALPN
00 10
00 0a // len
00 08 // len
07 // len
70 69 63 6f 74 6c 73 // picotls

// extension 1
12 34
00 0a // len
65 78 74 5f 31 5f 64 61 74 61 // ext_1_data

// extension 2
56 78
00 11 // len
65 78 74 5f 32 5f 73 65 72 76 // ext_2_server_data
65 72 5f 64 61 74 61

# encrypted
17 // PTLS_CONTENT_TYPE_APPDATA
03 03 // PTLS_PROTOCOL_VERSION_TLS12
00 4c // len

5e // PTLS_HANDSHAKE_TYPE_ENCRYPTED_EXTENSIONS
2f 63 fd // len

// extensions
4d 6f // len

// PTLS_EXTENSION_TYPE_SERVER_NAME
58 cd
31 8f

// PTLS_EXTENSION_TYPE_ALPN
60 ec
9d 07 // len
63 f1 // len
86 // len
ba ce e0 cb 34 3c e3 // picotls

// extension 1
b8 e7
73 ab // len
67 35 3f 89 ff ae 81 2a f7 1c // ext_1_data

// extension 2
db 2a
ad 07 // len
64 b7 07 1f 8c 92 16 e0 2f 14 // ext_2_server_data
71 5e bb fa 19 d6 cd

1f // PTLS_CONTENT_TYPE_HANDSHAKE

// tag
a7 0f 61 78 44 25 7a a6 e0 db
3f 20 96 67 26 80