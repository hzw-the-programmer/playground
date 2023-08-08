// record header
17 // PTLS_CONTENT_TYPE_APPDATA
03 03 // PTLS_PROTOCOL_VERSION_TLS12
00 23 // len

# encrypted message

49 d7 f8 5e 9f d5 a1 ef 57 5e
9d 5c 95 d4 aa 3a 26 60

1c // PTLS_CONTENT_TYPE_APPDATA

// tag
33 bd 57 ae e7 79 eb fd 49 31
06 16 2f 21 17 66

# decrypted message

47 45 54 20 2f 20 48 54 54 50 // GET / HTTP/1.1\r\n\r\n
2f 31 2e 31 0d 0a 0d 0a

17 // PTLS_CONTENT_TYPE_APPDATA