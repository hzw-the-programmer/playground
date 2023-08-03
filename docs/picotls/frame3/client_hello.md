16 // 
03 03 00 d0

01 00 00 cc

// legacy_version
03 03

// random_bytes
69 ca 39 44 d7 50 92 02 96 9b
a9 2b 79 bb 3e c9 49 e6 3c 01
0d e7 2c 3d 0f 13 11 c4 41 ab
19 fb

// lecagy_session_id
00

// cipher_suites
00 06
13 02 // PTLS_CIPHER_SUITE_AES_256_GCM_SHA384
13 01 // PTLS_CIPHER_SUITE_AES_128_GCM_SHA256
13 03 // PTLS_CIPHER_SUITE_CHACHA20_POLY1305_SHA256

// legacy_compression_methods
01 // len
00

// extensions
00 9d // len

// PTLS_EXTENSION_TYPE_KEY_SHARE
00 33 // ext type
00 47 // len
00 45 // len
00 17 // PTLS_GROUP_SECP256R1
00 41 // len
04
53 cb d6 92 7b e9 3d 6c 10 0a
75 cd cb 46 66 56 86 35 60 bc
d0 ab ad d7 33 56 6e ef b5 8e
cd 3c 74 4a 96 5b 4b 6b e2 6d
0c 3d 05 37 78 32 f0 46 c2 51
6e b9 2e 92 77 a2 ea fc f0 cd
81 54 ba 49

// PTLS_EXTENSION_TYPE_SERVER_NAME
00 00 // ext type
00 15 // len
00 13 // len
00 // PTLS_SERVER_NAME_TYPE_HOSTNAME
00 10 // len
74 65 73 74 2e 65 78 61 6d 70
6c 65 2e 63 6f 6d

// PTLS_EXTENSION_TYPE_ALPN
00 10 // ext type
00 11 // len
00 0f // len
06 // len
67 72 65 61 73 65 // grease
07 // len
70 69 63 6f 74 6c 73 // picotls

// PTLS_EXTENSION_TYPE_SUPPORTED_VERSIONS
00 2b // ext type
00 03 // len
02 // len
03 04 // PTLS_PROTOCOL_VERSION_TLS13

// PTLS_EXTENSION_TYPE_SIGNATURE_ALGORITHMS
00 0d // ext type
00 0a // len
00 08 // len
08 04 // PTLS_SIGNATURE_RSA_PSS_RSAE_SHA256
04 03 // PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256
04 01 // PTLS_SIGNATURE_RSA_PKCS1_SHA256
02 01 // PTLS_SIGNATURE_RSA_PKCS1_SHA1

// PTLS_EXTENSION_TYPE_SUPPORTED_GROUPS
00 0a // ext type
00 04 // len
00 02 // len
00 17 // PTLS_GROUP_SECP256R1

// additional extensions
04 d2 // ext type
00 03 // len
01 02 03