16 03 03 00 7b

02 00 00 77

// legacy_version
03 03

// random_bytes
ab 7c 9f c2 00 20 98 35 cc ee
a0 3f 80 33 3a 21 c0 51 6b 7c
a4 da da 70 05 e3 22 f9 ce c2
0e a2

// lecagy_session_id
00 // len

// cipher_suite
13 02 // PTLS_CIPHER_SUITE_AES_256_GCM_SHA384

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
81 b6 45 65 40 f8 87 8d 78 7c
be 3d 67 4f 40 d8 3a 73 6e 5b
45 da d2 d6 3e 8e 79 7a a2 3f
bf 03 ec 17 cc 64 68 f6 34 8c
78 2f 8e 41 2d 23 3e 18 a8 96
9c 9c 94 31 6e 7a 07 c1 9a 29
07 32 cf 8d