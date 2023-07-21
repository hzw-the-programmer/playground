00 00 00 00 00 00 00 00 // in_ctr
16 // record type: MBEDTLS_SSL_MSG_HANDSHAKE (22) in_hdr in_msgtype
03 03 // record version: MBEDTLS_SSL_VERSION_TLS1_2 (0x0303)
00 3b // record len

02
00 00 37
03 03 // server_version: MBEDTLS_SSL_VERSION_TLS1_2
0f fa 91 83 // Unix time
83 ce 7a c3 0d d6 4d 21 b7 af 30 6f 19 54 ba 59 16 25 97 76 44 4f 57 4e 47 52 44 01 // random
00 // session_id length
cc a8 // ciphersuite: TLS-ECDHE-RSA-WITH-CHACHA20-POLY1305-SHA256
00 // compression
00 0f // ext_len
00 23 // ext_id: MBEDTLS_TLS_EXT_SESSION_TICKET
00 00 // ext_size
ff 01  // ext_id: MBEDTLS_TLS_EXT_RENEGOTIATION_INFO
00 01 // ext_size
00
00 0b // ext_id: MBEDTLS_TLS_EXT_SUPPORTED_POINT_FORMATS
00 02 // ext_size
01 // list_size
00 // MBEDTLS_ECP_PF_UNCOMPRESSED
