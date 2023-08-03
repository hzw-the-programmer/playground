16 03 03 00 e7

01 00 00 e3

// legacy_version
03 03

// random_bytes
13 c2 6c 81 d9 29 1d 76 ba c9
0f de 6e 14 36 4d 92 56 6c 18
50 62 87 bc 6b f6 31 c0 3b 07
e6 31

// lecagy_session_id
00 // len

// cipher_suites
00 06
13 02 // PTLS_CIPHER_SUITE_AES_256_GCM_SHA384
13 01 // PTLS_CIPHER_SUITE_AES_128_GCM_SHA256
13 03 // PTLS_CIPHER_SUITE_CHACHA20_POLY1305_SHA256

// legacy_compression_methods
01
00

// extensions
00 b4

// PTLS_EXTENSION_TYPE_KEY_SHARE
00 33
00 47 // len
00 45 // len
00 17 //
00 41 // len
04
5f 5c ca 43 44 51 4a b3 cc 5c
a8 7a 8b 1b 0f d8 35 19 71 d4
76 92 bd 11 4a eb 2a 45 04 36
2a 58 1d 2f 9d e6 d8 fd e7 e1
88 8b 6c 84 c9 ee 96 61 38 ba
26 f1 a4 63 e1 cf b1 53 de f1
58 71 3d f1

// PTLS_EXTENSION_TYPE_SERVER_NAME
00 00
00 15 // len
00 13 // len
00 // PTLS_SERVER_NAME_TYPE_HOSTNAME
00 10 // len
74 65 73 74 2e 65 78 61 6d 70 // test.example.com
6c 65 2e 63 6f 6d

// PTLS_EXTENSION_TYPE_ALPN
00 10
00 11 // len
00 0f // len
06 // len
67 72 65 61 73 65 // grease
07 // len
70 69 63 6f 74 6c 73 // picotls

// PTLS_EXTENSION_TYPE_SUPPORTED_VERSIONS
00 2b
00 03 // len
02 // len
03 04 // PTLS_PROTOCOL_VERSION_TLS13

// PTLS_EXTENSION_TYPE_SIGNATURE_ALGORITHMS
00 0d
00 0a // len
00 08 // len
08 04 // PTLS_SIGNATURE_RSA_PSS_RSAE_SHA256
04 03 // PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256
04 01 // PTLS_SIGNATURE_RSA_PKCS1_SHA256
02 01 // PTLS_SIGNATURE_RSA_PKCS1_SHA1

// PTLS_EXTENSION_TYPE_SUPPORTED_GROUPS
00 0a
00 04 // len
00 02 // len
00 17 // PTLS_GROUP_SECP256R1

// additional extension
12 34
00 0a // len
65 78 74 5f 31 5f 64 61 74 61 // ext_1_data

// additional extension
56 78
00 0c // len
65 78 74 5f 32 5f 63 5f 64 61 // ext_2_c_data
74 61