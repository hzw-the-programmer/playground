// record header, 5 bytes
0x17, // PTLS_CONTENT_TYPE_APPDATA
0x03, 0x03, // PTLS_PROTOCOL_VERSION_TLS12
0x00, 0x23, // len

// encrypted message
0x5d, 0x36, 0x25, 0x9d, 0xfd, 0x84, 0x2c, 0x42, 0x17, 0x11,
0x3e, 0x6f, 0xcc, 0x9f, 0x97, 0xe6, 0x80, 0xe4,

// encrypted record type
0x17,

// tag, 16 bytes
0xde, 0xbd, 0x3b, 0x0b, 0x24, 0xa3, 0x16, 0xef, 0x71, 0xe8,
0xe0, 0xee, 0xd8, 0x87, 0x87, 0x7f,

### decrypted

// decrypted message: "GET / HTTP/1.1\r\n\r\n"
0x47, 0x45, 0x54, 0x20, 0x2f, 0x20, 0x48, 0x54, 0x54, 0x50,
0x2f, 0x31, 0x2e, 0x31, 0x0d, 0x0a, 0x0d, 0x0a,

// decrypted record type
0x17