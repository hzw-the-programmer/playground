16 // PTLS_CONTENT_TYPE_HANDSHAKE
03 // PTLS_RECORD_VERSION_MAJOR
03 // PTLS_RECORD_VERSION_MINOR
00 bb // len

01 // PTLS_HANDSHAKE_TYPE_CLIENT_HELLO
00 00 b7 // len

// legacy_version
03 03 // PTLS_PROTOCOL_VERSION_TLS12

// random_bytes
1b 5e 6e c1 2e 43 89 58 49 53
46 f8 ad 9b fc 98 8e 5f d8 a9
09 02 65 3e bc 2f 12 99 a0 34
20 3d

// legacy_session_id
00 // len

// cipher_suites
00 06 // len
13 02 // PTLS_CIPHER_SUITE_AES_256_GCM_SHA384
13 01 // PTLS_CIPHER_SUITE_AES_128_GCM_SHA256
13 03 // PTLS_CIPHER_SUITE_CHACHA20_POLY1305_SHA256

// legacy_compression_methods
01 // len
00

// extensions
00 88 // len

00 33 // PTLS_EXTENSION_TYPE_KEY_SHARE
00 47 // len
00 45 // len
00 17 00 41 04 ef 7e 6a 0e 81
68 91 3c c7 6a dc 76 5b ff 66
01 83 b6 9f c7 3b a2 38 85 b7
e0 7a c9 a1 0f e2 bf 9d 3f a8
c2 af 25 94 a7 64 c1 eb ef d4
5d 4d 63 51 e5 9d d4 07 cb 10
0e bc 3b 8f f0 41 50 24 52

00 00 // PTLS_EXTENSION_TYPE_SERVER_NAME
00 15 // len
00 13 // len
00 // PTLS_SERVER_NAME_TYPE_HOSTNAME
00 10 // len
74 65 73 74 2e 65 78 61 6d 70 6c 65 2e 63 6f 6d // test.example.com

00 2b // PTLS_EXTENSION_TYPE_SUPPORTED_VERSIONS
00 03 // len
02 // len
03 04 // PTLS_PROTOCOL_VERSION_TLS13

00 0d // PTLS_EXTENSION_TYPE_SIGNATURE_ALGORITHMS
00 0a // len
00 08 // len
08 04 // PTLS_SIGNATURE_RSA_PSS_RSAE_SHA256
04 03 // PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256
04 01 // PTLS_SIGNATURE_RSA_PKCS1_SHA256
02 01 // PTLS_SIGNATURE_RSA_PKCS1_SHA1

00 0a // PTLS_EXTENSION_TYPE_SUPPORTED_GROUPS
00 04 // len
00 02 // len
00 17 // PTLS_GROUP_SECP256R1

04 d2 // 1234
00 03 // len
01 02 03