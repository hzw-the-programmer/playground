17 // PTLS_CONTENT_TYPE_APPDATA
03 03 // PTLS_PROTOCOL_VERSION_TLS12
00 35 // len

# encrypted message

23 // PTLS_HANDSHAKE_TYPE_FINISHED
98 e3 f6 // len

// verify_data
c3 0a ae cb 36 94 22 7a f1 35
f7 52 f1 56 55 97 ce ea a9 19
4c dd d4 ac 5b 55 30 7a c2 58
c4 f3

2a // PTLS_CONTENT_TYPE_HANDSHAKE

// tag
f0 5c 25 9c 54 4a fd f1 f1 4f
72 cd 95 c9 db cb

# decrypted message

14 // PTLS_HANDSHAKE_TYPE_FINISHED
00 00 20 // len

// verify_data
d3 02 dc e6 77 c6 08 3b 18 d8
77 b6 e4 6e cc 90 52 ec aa 21
b7 ba 7f 39 66 a1 60 0d d6 d2
ee bf

16 // PTLS_CONTENT_TYPE_HANDSHAKE