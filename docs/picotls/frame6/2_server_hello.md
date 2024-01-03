// record header
16 // PTLS_CONTENT_TYPE_HANDSHAKE
03 03 // PTLS_PROTOCOL_VERSION_TLS12
00 7b // len

// message header
02 // PTLS_HANDSHAKE_TYPE_SERVER_HELLO
00 00 77 // len

// legacy_version
03 03 // PTLS_PROTOCOL_VERSION_TLS12

// random_bytes
75 82 32 4d 9d b6 75 5a 8f e6
b5 a9 47 3f 00 8b 21 79 16 91
91 98 c1 c5 0b 36 9d 57 a2 0c
c9 70

// lecagy_session_id
00 // len

// select cipher_suite
13 01 // PTLS_CIPHER_SUITE_AES_128_GCM_SHA256

// legacy_compression_method
00

// extensions
00 4f // len

// PTLS_EXTENSION_TYPE_SUPPORTED_VERSIONS
00 2b
00 02 // len
03 04 // PTLS_PROTOCOL_VERSION_TLS13

// PTLS_EXTENSION_TYPE_KEY_SHARE
00 33
00 45 // len
00 17 // PTLS_GROUP_SECP256R1
00 41 // len
04 // TYPE_UNCOMPRESSED_PUBLIC_KEY
33 d2 56 4c a3 50 fa 4e 4a f7
7f 76 ba f1 02 72 9e bd 2f 62
c1 ef 06 ed 0f 5a 1a 7e b9 6f
25 3c 05 57 98 68 77 ed 83 1a
95 c2 ab b6 b8 0d 8a ac f4 a2
b2 c1 f3 9b ab 2c 74 e2 00 75
aa 8b b9 8a