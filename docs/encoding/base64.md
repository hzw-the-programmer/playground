input: f
    0x66
    01100110
    00011001 00100000
    0x19 0x20 ==
    25 32 ==
output: Zg==

input: fo
    0x66 0x6f
    01100110 01101111
    00011001 00100110 00111100
    0x19 0x26 0x3c =
    25 38 60 =
output: Zm8=

input: foo
    0x66 0x6f 0x6f
    01100110 01101111 01101111
    00011001 00100110 00111101 00101111
    0x19 0x26 0x3d 0x2f
    25 38 61 47
output: Zm9v

Table 1: The Base 64 Alphabet
Value Encoding  Value Encoding  Value Encoding  Value Encoding
0 A            17 R            34 i            51 z
1 B            18 S            35 j            52 0
2 C            19 T            36 k            53 1
3 D            20 U            37 l            54 2
4 E            21 V            38 m            55 3
5 F            22 W            39 n            56 4
6 G            23 X            40 o            57 5
7 H            24 Y            41 p            58 6
8 I            25 Z            42 q            59 7
9 J            26 a            43 r            60 8
10 K            27 b            44 s            61 9
11 L            28 c            45 t            62 +
12 M            29 d            46 u            63 /
13 N            30 e            47 v
14 O            31 f            48 w         (pad) =
15 P            32 g            49 x
16 Q            33 h            50 y

Table 2: The "URL and Filename safe" Base 64 Alphabet
Value Encoding  Value Encoding  Value Encoding  Value Encoding
0 A            17 R            34 i            51 z
1 B            18 S            35 j            52 0
2 C            19 T            36 k            53 1
3 D            20 U            37 l            54 2
4 E            21 V            38 m            55 3
5 F            22 W            39 n            56 4
6 G            23 X            40 o            57 5
7 H            24 Y            41 p            58 6
8 I            25 Z            42 q            59 7
9 J            26 a            43 r            60 8
10 K            27 b            44 s            61 9
11 L            28 c            45 t            62 - (minus)
12 M            29 d            46 u            63 _
13 N            30 e            47 v           (underline)
14 O            31 f            48 w
15 P            32 g            49 x
16 Q            33 h            50 y         (pad) =

https://datatracker.ietf.org/doc/html/rfc4648#section-4