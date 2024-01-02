// record header
17 // PTLS_CONTENT_TYPE_APPDATA
03 03 // PTLS_PROTOCOL_VERSION_TLS12
00 b4 // len

# encrypted message

52 bc ab 6c bb b3 27 e1 36 19
68 e0 33 2b 12 88 ef a8 c6 e2
23 e1 dd 5b 55 3d e4 65 c4 e1
7a ce 91 7b 97 6a 2d 6e 2a de
1b ef 91 60 d5 c5 04 78 c9 83
0c bc 45 1c 53 e0 3a 16 7f 7e
89 cf 02 c9 f5 b9 34 a8 b7 a7
04 47 93 02 3f ad 3a a6 c1 56
00 be 77 b0 39 d0 1c a5 7a c0
14 99 4a 6c 83 34 be 01 aa 94

3d eb b9 aa 2a 3a cd 12 29 d0
f4 7e 22 7b fd ed 13 1b 5c dd
ca 5b 91 ee ca 98 12 f6 72 06
e0 0c 80 02 5f c8 47 a9 e6 ce
3e e3 11 78 78 13 98 f5 b3 fc
32 6e e1 15 31 b1 d4 30 28 1b
0a 2e 98

14 // PTLS_CONTENT_TYPE_APPDATA

// tag
e3 6c 5b 44 cb f5 65 3d 6c 6d
77 ce 64 0e f2 69

# decrypted message

48 54 54 50 2f 31 2e 31 20 34
30 30 20 42 61 64 20 52 65 71
75 65 73 74 3a 20 6d 69 73 73
69 6e 67 20 72 65 71 75 69 72
65 64 20 48 6f 73 74 20 68 65
61 64 65 72 0d 0a 43 6f 6e 74
65 6e 74 2d 54 79 70 65 3a 20
74 65 78 74 2f 70 6c 61 69 6e
3b 20 63 68 61 72 73 65 74 3d
75 74 66 2d 38 0d 0a 43 6f 6e

6e 65 63 74 69 6f 6e 3a 20 63
6c 6f 73 65 0d 0a 0d 0a 34 30
30 20 42 61 64 20 52 65 71 75
65 73 74 3a 20 6d 69 73 73 69
6e 67 20 72 65 71 75 69 72 65
64 20 48 6f 73 74 20 68 65 61
64 65 72

17 // PTLS_CONTENT_TYPE_APPDATA

HTTP/1.1 400 Bad Request: missing required Host header\r\n
Content-Type: text/plain; charset=utf-8\r\n
Connection: close\r\n\r\n
400 Bad Request: missing required Host header