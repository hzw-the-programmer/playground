16 03 03 00 bb

01 // PTLS_HANDSHAKE_TYPE_CLIENT_HELLO
00 00 b7

// legacy_version
03 03

// random_bytes
c4 4a 58 ea c2 ea e6 c7 f9 6a
83 78 83 55 26 51 5c da ff f5
37 40 d0 8b ce 51 45 00 ff bb
56 39

// lecagy_session_id
00

// cipher_suites
00 06
13 02
13 01
13 03

// legacy_compression_methods
01
00

// extensions
00 88

// PTLS_EXTENSION_TYPE_KEY_SHARE
00 33
00 47
00 45
00 17 // PTLS_GROUP_SECP256R1
00 41
04 // TYPE_UNCOMPRESSED_PUBLIC_KEY
29 f0 ee 7d e8 25 d7 f5 a3 1f
5a 01 ab 18 67 64 19 fe 15 30
a5 59 f0 fc 16 38 ce 86 8b 63
1a fd cd 66 4e 54 72 d1 d8 ee
ce 5c bf 87 7f 0c 32 bd b7 64
17 e7 c5 3a af 18 50 79 2e cb
35 55 ec bd

// PTLS_EXTENSION_TYPE_SERVER_NAME
00 00
00 15
00 13
00 // PTLS_SERVER_NAME_TYPE_HOSTNAME
00 10
74 65 73 74 2e 65 78 61 6d 70 // test.example.com
6c 65 2e 63 6f 6d

// PTLS_EXTENSION_TYPE_SUPPORTED_VERSIONS
00 2b
00 03
02
03 04 // PTLS_PROTOCOL_VERSION_TLS13

// PTLS_EXTENSION_TYPE_SIGNATURE_ALGORITHMS
00 0d
00 0a
00 08
08 04 // PTLS_SIGNATURE_RSA_PSS_RSAE_SHA256
04 03 // PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256
04 01 // PTLS_SIGNATURE_RSA_PKCS1_SHA256
02 01 // PTLS_SIGNATURE_RSA_PKCS1_SHA1

// PTLS_EXTENSION_TYPE_SUPPORTED_GROUPS
00 0a
00 04
00 02
00 17 // PTLS_GROUP_SECP256R1

// additional_extensions
 04 d2
 00 03
 01 02 03