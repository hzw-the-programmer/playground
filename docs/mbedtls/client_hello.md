mbedtls_ssl_write_client_hello
mbedtls_ssl_start_handshake_msg
ssl_write_client_hello_body

out_ctr
out_buf
00 00 00 00 00 00 00 00
out_hdr
16 // MBEDTLS_SSL_MSG_HANDSHAKE 22
03 03 // MBEDTLS_SSL_VERSION_TLS1_2
out_len
01 49
out_msg
01 // MBEDTLS_SSL_HS_CLIENT_HELLO 1
00 01 45

// ver (2)
03 03 // MBEDTLS_SSL_VERSION_TLS1_2

// gmt (4)
64 b8 e0 c6
// random (28)
d3 69 d3 b3 23 d9 df d7 59 01 56 48 16 34 64 39 77 4d 2b 74 02 ad 2e c7 07 d3 5b 5f

// id len (1)
00

// cypher len (2)
00 d0
// TLS-ECDHE-RSA-WITH-CHACHA20-POLY1305-SHA256
cc a8
// TLS-ECDHE-ECDSA-WITH-CHACHA20-POLY1305-SHA256
cc a9
// TLS-DHE-RSA-WITH-CHACHA20-POLY1305-SHA256
cc aa
// TLS-ECDHE-ECDSA-WITH-AES-256-GCM-SHA384
c0 2c
// TLS-ECDHE-RSA-WITH-AES-256-GCM-SHA384
c0 30
// TLS-DHE-RSA-WITH-AES-256-GCM-SHA384
00 9f
// TLS-ECDHE-ECDSA-WITH-AES-256-CCM
c0 ad
// TLS-DHE-RSA-WITH-AES-256-CCM
c0 9f
// TLS-ECDHE-ECDSA-WITH-AES-256-CBC-SHA384
c0 24
// TLS-ECDHE-RSA-WITH-AES-256-CBC-SHA384
c0 28
// TLS-DHE-RSA-WITH-AES-256-CBC-SHA256
00 6b
// TLS-ECDHE-ECDSA-WITH-AES-256-CBC-SHA
c0 0a
// TLS-ECDHE-RSA-WITH-AES-256-CBC-SHA
c0 14
// TLS-DHE-RSA-WITH-AES-256-CBC-SHA
00 39
// TLS-ECDHE-ECDSA-WITH-AES-256-CCM-8
c0 af
// 
c0 a3
// TLS-ECDHE-ECDSA-WITH-CAMELLIA-256-GCM-SHA384
c0 87
// TLS-ECDHE-RSA-WITH-CAMELLIA-256-GCM-SHA384
c0 8b
// TLS-DHE-RSA-WITH-CAMELLIA-256-GCM-SHA384
c0 7d
// TLS-ECDHE-ECDSA-WITH-CAMELLIA-256-CBC-SHA384
c0 73
// TLS-ECDHE-RSA-WITH-CAMELLIA-256-CBC-SHA384
c0 77
// TLS-DHE-RSA-WITH-CAMELLIA-256-CBC-SHA256
00 c4
// TLS-DHE-RSA-WITH-CAMELLIA-256-CBC-SHA
00 88
// TLS-ECDHE-ECDSA-WITH-ARIA-256-GCM-SHA384
c0 5d
// TLS-ECDHE-RSA-WITH-ARIA-256-GCM-SHA384
c0 61
// TLS-DHE-RSA-WITH-ARIA-256-GCM-SHA384
c0 53
// TLS-ECDHE-ECDSA-WITH-ARIA-256-CBC-SHA384
c0 49
// TLS-ECDHE-RSA-WITH-ARIA-256-CBC-SHA384
c0 4d
// TLS-DHE-RSA-WITH-ARIA-256-CBC-SHA384
c0 45
// TLS-ECDHE-ECDSA-WITH-AES-128-GCM-SHA256
c0 2b
// TLS-ECDHE-RSA-WITH-AES-128-GCM-SHA256
c0 2f
// TLS-DHE-RSA-WITH-AES-128-GCM-SHA256
00 9e
// TLS-ECDHE-ECDSA-WITH-AES-128-CCM
c0 ac
// TLS-DHE-RSA-WITH-AES-128-CCM
c0 9e
// TLS-ECDHE-ECDSA-WITH-AES-128-CBC-SHA256
c0 23
// TLS-ECDHE-RSA-WITH-AES-128-CBC-SHA256
c0 27
// TLS-DHE-RSA-WITH-AES-128-CBC-SHA256
00 67
// TLS-ECDHE-ECDSA-WITH-AES-128-CBC-SHA
c0 09
// TLS-ECDHE-RSA-WITH-AES-128-CBC-SHA
c0 13
// TLS-DHE-RSA-WITH-AES-128-CBC-SHA
00 33
// TLS-ECDHE-ECDSA-WITH-AES-128-CCM-8
c0 ae
// TLS-DHE-RSA-WITH-AES-128-CCM-8
c0 a2
// TLS-ECDHE-ECDSA-WITH-CAMELLIA-128-GCM-SHA256
c0 86
// TLS-ECDHE-RSA-WITH-CAMELLIA-128-GCM-SHA256
c0 8a
// TLS-DHE-RSA-WITH-CAMELLIA-128-GCM-SHA256
c0 7c
// TLS-ECDHE-ECDSA-WITH-CAMELLIA-128-CBC-SHA256
c0 72
// TLS-ECDHE-RSA-WITH-CAMELLIA-128-CBC-SHA256
c0 76
// TLS-DHE-RSA-WITH-CAMELLIA-128-CBC-SHA256
00 be
// TLS-DHE-RSA-WITH-CAMELLIA-128-CBC-SHA
00 45
// TLS-ECDHE-ECDSA-WITH-ARIA-128-GCM-SHA256
c0 5c
// TLS-ECDHE-RSA-WITH-ARIA-128-GCM-SHA256
c0 60
// TLS-DHE-RSA-WITH-ARIA-128-GCM-SHA256
c0 52
// TLS-ECDHE-ECDSA-WITH-ARIA-128-CBC-SHA256
c0 48
// TLS-ECDHE-RSA-WITH-ARIA-128-CBC-SHA256
c0 4c
// TLS-DHE-RSA-WITH-ARIA-128-CBC-SHA256
c0 44
// TLS-RSA-WITH-AES-256-GCM-SHA384
00 9d
// TLS-RSA-WITH-AES-256-CCM
c0 9d
// TLS-RSA-WITH-AES-256-CBC-SHA256
00 3d
// TLS-RSA-WITH-AES-256-CBC-SHA
00 35
// TLS-ECDH-RSA-WITH-AES-256-GCM-SHA384
c0 32
// TLS-ECDH-RSA-WITH-AES-256-CBC-SHA384
c0 2a
// TLS-ECDH-RSA-WITH-AES-256-CBC-SHA
c0 0f
// TLS-ECDH-ECDSA-WITH-AES-256-GCM-SHA384
c0 2e
// TLS-ECDH-ECDSA-WITH-AES-256-CBC-SHA384
c0 26
// TLS-ECDH-ECDSA-WITH-AES-256-CBC-SHA
c0 05
// TLS-RSA-WITH-AES-256-CCM-8
c0 a1
// TLS-RSA-WITH-CAMELLIA-256-GCM-SHA384
c0 7b
// TLS-RSA-WITH-CAMELLIA-256-CBC-SHA256
00 c0
// TLS-RSA-WITH-CAMELLIA-256-CBC-SHA
00 84
// TLS-ECDH-RSA-WITH-CAMELLIA-256-GCM-SHA384
c0 8d
// TLS-ECDH-RSA-WITH-CAMELLIA-256-CBC-SHA384
c0 79
// TLS-ECDH-ECDSA-WITH-CAMELLIA-256-GCM-SHA384
c0 89
// TLS-ECDH-ECDSA-WITH-CAMELLIA-256-CBC-SHA384
c0 75
// TLS-ECDH-ECDSA-WITH-ARIA-256-GCM-SHA384
c0 5f
// TLS-ECDH-RSA-WITH-ARIA-256-GCM-SHA384
c0 63
// TLS-RSA-WITH-ARIA-256-GCM-SHA384
c0 51
// TLS-ECDH-ECDSA-WITH-ARIA-256-CBC-SHA384
c0 4b
// TLS-ECDH-RSA-WITH-ARIA-256-CBC-SHA384
c0 4f
// TLS-RSA-WITH-ARIA-256-CBC-SHA384
c0 3d
// TLS-RSA-WITH-AES-128-GCM-SHA256
00 9c
// TLS-RSA-WITH-AES-128-CCM
c0 9c
// TLS-RSA-WITH-AES-128-CBC-SHA256
00 3c
// TLS-RSA-WITH-AES-128-CBC-SHA
00 2f
// TLS-ECDH-RSA-WITH-AES-128-GCM-SHA256
c0 31
// TLS-ECDH-RSA-WITH-AES-128-CBC-SHA256
c0 29
// TLS-ECDH-RSA-WITH-AES-128-CBC-SHA
c0 0e
// TLS-ECDH-ECDSA-WITH-AES-128-GCM-SHA256
c0 2d
// TLS-ECDH-ECDSA-WITH-AES-128-CBC-SHA256
c0 25
// TLS-ECDH-ECDSA-WITH-AES-128-CBC-SHA
c0 04
// TLS-RSA-WITH-AES-128-CCM-8
c0 a0
// TLS-RSA-WITH-CAMELLIA-128-GCM-SHA256
c0 7a
// TLS-RSA-WITH-CAMELLIA-128-CBC-SHA256
00 ba
// TLS-RSA-WITH-CAMELLIA-128-CBC-SHA
00 41
// TLS-ECDH-RSA-WITH-CAMELLIA-128-GCM-SHA256
c0 8c
// TLS-ECDH-RSA-WITH-CAMELLIA-128-CBC-SHA256
c0 78
// TLS-ECDH-ECDSA-WITH-CAMELLIA-128-GCM-SHA256
c0 88
// TLS-ECDH-ECDSA-WITH-CAMELLIA-128-CBC-SHA256
c0 74
// TLS-ECDH-ECDSA-WITH-ARIA-128-GCM-SHA256
c0 5e
// TLS-ECDH-RSA-WITH-ARIA-128-GCM-SHA256
c0 62
// TLS-RSA-WITH-ARIA-128-GCM-SHA256
c0 50
// TLS-ECDH-ECDSA-WITH-ARIA-128-CBC-SHA256
c0 4a
// TLS-ECDH-RSA-WITH-ARIA-128-CBC-SHA256
c0 4e
// TLS-RSA-WITH-ARIA-128-CBC-SHA256
c0 3c

// EMPTY_RENEGOTIATION_INFO_SCSV
00 ff

// compress len
01
// MBEDTLS_SSL_COMPRESS_NULL
00

// extension len
00 4c

// extension_type (2): MBEDTLS_TLS_EXT_SERVERNAME
00 00
// extension_data_length (2): 2 + 1 + 2 + hostname len
00 0e
// 1 + 2 + hostname len
00 0c
// MBEDTLS_TLS_EXT_SERVERNAME_HOSTNAME
00
// hostname len
00 09
// hostname
6c 6f 63 61 6c 68 6f 73 74

// extension_type (2): MBEDTLS_TLS_EXT_SUPPORTED_GROUPS
00 0a
// extension_data_length (2)
00 12
// named_group_list_length (2)
00 10
// MBEDTLS_SSL_IANA_TLS_GROUP_X25519
00 1d
// MBEDTLS_SSL_IANA_TLS_GROUP_SECP256R1
00 17
// MBEDTLS_SSL_IANA_TLS_GROUP_SECP384R1
00 18
// MBEDTLS_SSL_IANA_TLS_GROUP_X448
00 1e
// MBEDTLS_SSL_IANA_TLS_GROUP_SECP521R1
00 19
// MBEDTLS_SSL_IANA_TLS_GROUP_BP256R1
00 1a
// MBEDTLS_SSL_IANA_TLS_GROUP_BP384R1
00 1b
// MBEDTLS_SSL_IANA_TLS_GROUP_BP512R1
00 1c

// extension_type (2): MBEDTLS_TLS_EXT_SIG_ALG
00 0d
// extension_data_length (2)
00 0e
// supported_signature_algorithms_length (2)
00 0c
// MBEDTLS_TLS1_3_SIG_ECDSA_SECP521R1_SHA512
06 03
// MBEDTLS_TLS1_3_SIG_RSA_PKCS1_SHA512
06 01
// MBEDTLS_TLS1_3_SIG_ECDSA_SECP384R1_SHA384
05 03
// MBEDTLS_TLS1_3_SIG_RSA_PKCS1_SHA384
05 01
// MBEDTLS_TLS1_3_SIG_ECDSA_SECP256R1_SHA256
04 03
// MBEDTLS_TLS1_3_SIG_RSA_PKCS1_SHA256
04 01

// extension_type (2): MBEDTLS_TLS_EXT_SUPPORTED_POINT_FORMATS
00 0b
// len
00 02
// compress len
01
// MBEDTLS_ECP_PF_UNCOMPRESSED
00

// extension_type (2): MBEDTLS_TLS_EXT_ENCRYPT_THEN_MAC
00 16
// len
00 00

// extension_type (2): MBEDTLS_TLS_EXT_EXTENDED_MASTER_SECRET
00 17
// len
00 00

// extension_type (2): MBEDTLS_TLS_EXT_SESSION_TICKET
00 23
// len
00 00