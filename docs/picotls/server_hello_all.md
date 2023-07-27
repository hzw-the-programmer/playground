# server_hello begin
16 // PTLS_CONTENT_TYPE_HANDSHAKE
03 03
00 7b

02
00 00 77

03 03

66 04 06 12 de 05 2c 6c 79 db
66 f5 3a 5b 96 87 8f 70 df 96
57 ba ae e2 4d df 39 35 33 bf
c6 88

00

13 02

00

00 4f

00 2b
00 02
03 04

00 33
00 45
00 17
00 41
04 8c 19 40 35 62 cb be 9b 51
d3 4a d6 da 2a c6 c7 d0 c6 d6
52 4e 98 b8 b6 a3 de 73 68 2c
28 79 3e c8 d8 0b 8b 8a f5 e3
0f f6 f6 8a b0 f5 b1 51 6a 36
5e 22 3e 03 62 7a b1 4c 64 19
d8 e5 d3 bb 98
# server_hello end

# encrypted_extensions begin
17 // PTLS_CONTENT_TYPE_APPDATA
03 03
00 17

66 47 d8 b3 a8 b1
49
b8 57 13 b6 45 f5 7c 4e 42 b6 77 bc dd 32 86 0a
# encrypted_extensions end

# server_certificate begin
17 // PTLS_CONTENT_TYPE_APPDATA
03 // PTLS_RECORD_VERSION_MAJOR
03 // PTLS_RECORD_VERSION_MINOR
02 82 // len

## encrypted content begin
10 4f 4c 25 a0 15 8e b9 c4 84
b4 32 85 34 34 f5 8d e6 8e dc
8e 58 af 56 41 0a 19 f7 85 52
e0 89 75 0f 26 66 83 54 6e aa
e0 0e 36 79 34 63 65 19 05 84
43 4f 21 f1 43 25 7d d9 9d 72
43 d3 8f 7f b0 5e 4e 97 15 36
ae 33 9f 1c 4a d8 48 70 8c 6d
00 10 45 a1 58 52 d3 4c 56 67
81 e5 69 80 ab a6 f8 a2 4d 2a

6d 88 c9 01 34 0c 8e 0f fe d9
8b e9 f9 bf 7e 4b 2a 17 31 5b
4c 6e d3 db de b3 61 af 85 63
ba 66 39 1e b1 6b f0 41 14 cb
f6 de 4e cd 45 2c f8 33 fa 89
63 0c dd 1f 87 ce 3d d1 7f dd
2e a2 9a 8f 5c af 74 39 f7 1d
6e 96 c9 75 b4 6a 49 8f 81 1b
94 d7 da 5c 3c e1 25 8d bc 1d
84 7a c2 77 dd 03 0a 35 7c f0

e3 b9 cb 15 59 5d 9f 62 51 e1
17 2a ca 76 65 b7 66 0d 43 f5
ae 7b 98 de de 8b d3 8b 5b 2b
11 2b 5c 3f 01 3d 7b db 2f 58
b3 aa c3 c5 62 05 64 a8 b4 9a
db 7b 54 f4 56 3b 88 0f 44 26
91 54 94 d1 41 fc b7 bf 3e b0
73 bb ac 03 a9 4c c3 8c 1f 0d
9a f9 40 bd f3 fc 28 02 66 1e
b1 6b 26 fc 98 2e 5e 9d 0e fd

a1 4e 5f 23 96 17 50 93 3d 24
a7 09 91 7f 87 b6 32 d4 50 4d
dd d2 c4 be 15 43 cb 51 9b ae
ca a0 f8 cc 74 bc 43 5e a3 05
58 c4 a1 37 5d 3b a9 d7 7e 66
81 af 04 27 08 45 21 e4 d6 1e
66 58 7f df 39 8d 8b 2b 7f 4a
55 e8 dd f6 f3 ec 87 e0 5a a3
d1 d5 ef 01 af d4 2f e1 5c fd
d1 95 2e fb e6 c1 b9 0b c6 a7

12 26 da 3a 6a c9 30 06 2e aa
4d c7 8d 93 51 07 ff 74 c6 96
8b 64 6e a3 5c d7 6c c4 37 fb
24 56 1f a8 6f 0c 17 8b 64 3a
d4 d6 c7 88 13 f4 0e cd 2b e7
02 f0 bc ed ef bf aa d4 ea b7
3b 83 49 84 92 aa 6b d6 75 ff
96 5b 4c c3 56 65 19 b4 99 61
0d 83 fd 55 38 3f 7f f0 22 30
55 a0 36 69 c1 ec 81 66 8b 3e

72 b7 a6 25 fd d3 33 c6 da 8f
4f 9d 37 a7 ce a8 d4 68 5a 0b
95 c4 6f db 86 ad a9 36 c0 6f
f1 a4 21 ad c8 eb 1e b6 8f 6e
92 98 2a 38 47 0e 7f 25 87 80
e5 db 27 39 0e b6 dd 24 5b b7
08 23 ed 30 fa c6 81 85 07 39
11 cc 6f e7 09 3f ab 2e da 3b
07 79 53 65 c0 29 30 2e 1f c4
09 e2 79 6c 7a 90 ea e9 5d 42

59 1c 23 be ab 5d 8b c0 43 6b
96 49 84 3c 4b 79 51 06 ea b9
53 6a 2b 04 a1
## encrypted content end

## encryted content type: PTLS_CONTENT_TYPE_HANDSHAKE
83

## tag
4a ac 87 3f 2f 8f 47 1f 5a d8 4d ed bd 00 72 62
# server_certificate end

# certificate_verify begin
17 // PTLS_CONTENT_TYPE_APPDATA
03 // PTLS_RECORD_VERSION_MAJOR
03 // PTLS_RECORD_VERSION_MINOR
00 60 // len

## content begin
90 73 98 83 ac 5e 02 5a 42 5c
b0 02 83 02 1e a2 1d 5b 44 3d
03 61 c7 31 d9 b7 4f 3b b5 83
54 47 08 d6 d0 6e 89 b0 22 3e
30 89 89 fc 48 8f 44 c6 5e 2c
e5 cd 83 a0 01 63 2f 8f a2 4e
64 95 1d 4c 60 29 0a 14 7a 66
60 d7 2f d4 d9 60 a0 37 72
## content end

## encrypted type: PTLS_CONTENT_TYPE_HANDSHAKE
32

## tag
18 96 f9 0f 2e 47 ba ed 0c ee
f4 6e d4 0d ef 7b
# certificate_verify end

# finished begin
17 // PTLS_CONTENT_TYPE_APPDATA
03 // PTLS_RECORD_VERSION_MAJOR
03 // PTLS_RECORD_VERSION_MINOR
00 45 // len

## encrypted content begin
c4 43 76 56 dc 3a 72 6d 10 56
63 f5 b3 7a 3b b2 5d 66 31 5e
9b d2 3d a8 8e 57 75 d1 71 c4
b7 57 2a aa 81 e2 6f 47 83 51
6a 3b 10 52 77 60 13 af b1 0b
4a 88
## encrypted content end

## encrypted content type: PTLS_CONTENT_TYPE_HANDSHAKE
bf

## tag
89 b5 d3 e2 40 f6 ce 69 fc 48
55 c7 1a 84 c0 09
# finished end