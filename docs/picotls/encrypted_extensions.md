# encrypted_extensions begin
16 // PTLS_CONTENT_TYPE_HANDSHAKE
03 // PTLS_RECORD_VERSION_MAJOR
03 // PTLS_RECORD_VERSION_MINOR
00 00 // len

08 // PTLS_HANDSHAKE_TYPE_ENCRYPTED_EXTENSIONS
00 00 02
00 00
# encrypted_extensions end


# encrypted encrypted_extensions begin
// buffer_encrypt_record
17 // PTLS_CONTENT_TYPE_APPDATA
03 // PTLS_RECORD_VERSION_MAJOR
03 // PTLS_RECORD_VERSION_MINOR
00 17 // len

## encrypted content
66 47 d8 b3 a8 b1 // encrypted from: 08 00 00 02 00 00

## encrypted content type: PTLS_CONTENT_TYPE_HANDSHAKE
49

## tag
b8 57 13 b6 45 f5 7c 4e 42 b6 77 bc dd 32 86 0a
# encrypted encrypted_extensions end