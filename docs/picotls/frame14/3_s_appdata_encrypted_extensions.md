// record header, 5 bytes
0x17, // PTLS_CONTENT_TYPE_APPDATA
0x03, 0x03, // PTLS_PROTOCOL_VERSION_TLS12
0x00, 0x17, // len

// encrypted handshake message header, 4 bytes
0xf0, // encrypted handshake message type
0xb1, 0x29, 0x08, // encrypted len

// encrypted handshake message body
0x7e, 0xfe,

// encrypted record type
0xc0,

// tag, 16 bytes
0x40, 0xb4, 0x91, 0x7f, 0x45, 0x49, 0x12, 0xaf, 0x3a, 0x98,
0xfb, 0x3f, 0x2b, 0xc2, 0xad, 0x0a,

### decrypted

// decrypted handshake message header, 4 bytes
08 // PTLS_HANDSHAKE_TYPE_ENCRYPTED_EXTENSIONS
00 00 02 // len

// decrypted handshake message body
00 00

// decrypted record type
16