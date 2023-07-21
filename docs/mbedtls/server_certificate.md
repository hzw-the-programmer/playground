00 00 00 00 00 00 00 00

16 // record type: MBEDTLS_SSL_MSG_HANDSHAKE (22)
03 03
03 79

0b // MBEDTLS_SSL_HS_CERTIFICATE (11) in_msg
00 03 75 // len
00
03 72 // crt chain len
00 03 6f // crt len

30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE crt->raw.p
82 // len: 0x80 | 0x02: next two bytes: mbedtls_asn1_get_len
03 6b // two bytes len

30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE crt->tbs.p
82 // len: 0x80 | 0x02: next two bytes: mbedtls_asn1_get_len
02 53 // tow bytes len
a0 // tag: MBEDTLS_ASN1_CONTEXT_SPECIFIC | MBEDTLS_ASN1_CONSTRUCTED
03 // len
02 // tag: MBEDTLS_ASN1_INTEGER
01 // len
02 // version
02 // serial->tag
14 // serial->len
3e 70 1e 5d e2 dc bc e0 25 20 c0 d7 d0 56 e3 a7 2d 6e 43 c1 // serial->p
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
0d // len
06 // sig_oid->tag MBEDTLS_ASN1_OID
09 // sig_oid->len
2a 86 48 86 f7 0d 01 01 0b // sig_oid->p MBEDTLS_OID_PKCS1_SHA256 oid_sig_alg
        // 2a MBEDTLS_OID_ISO_MEMBER_BODIES
        // 86 48 MBEDTLS_OID_COUNTRY_US
        // 86 f7 0d MBEDTLS_OID_ORG_RSA_DATA_SECURITY
        // MBEDTLS_OID_RSA_COMPANY
        // 01 MBEDTLS_OID_PKCS
        // 01 MBEDTLS_OID_PKCS1
        // 0b MBEDTLS_OID_PKCS1_SHA256
05 // sig_params1->tag
00 // sig_params1->len

# issuer
## crt->issuer_raw.p begin
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
45 // len

31 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SET
0b // set_len
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
09 // len
06 // oid->tag MBEDTLS_ASN1_OID
03 // oid->len
55 04 06 // oid->p
13 // val->tag
02 // val->len
41 55 // // val->p AU

31 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SET
13 // set_len
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
11 // len
06 // oid->tag MBEDTLS_ASN1_OID
03 // oid->len
55 04 08 // oid->p
0c // value->tag
0a // val->len
53 6f 6d 65 2d 53 74 61 74 65 // val->p Some-State

31 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SET
21 // len
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
1f // len
06 // oid->tag MBEDTLS_ASN1_OID
03 // oid->len
55 04 0a // oid->p
0c // val->tag
18 // val->len
49 6e 74 65 72 6e 65 74 20 57
69 64 67 69 74 73 20 50 74 79
20 4c 74 64 // val->p Internet_Widgits_Pty_Ltd
## crt->issuer_raw.p end

# Validity
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
1e // len
17 // tag: MBEDTLS_ASN1_UTC_TIME
0d // len
32 33 // year 2023
30 37 // month 07
31 39 // day 19
31 31 // hour 11
33 37 // minute 37
30 38 // second 08
5a // 'Z'
17 // tag: MBEDTLS_ASN1_UTC_TIME
0d // len
33 33 // year 2033
30 37 // month 07
31 36 // day 16
31 31 // hour 11
33 37 // minute 37
30 38 // second 08
5a // 'Z'

# subject
## crt->subject_raw.p begin
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
45 // len

31 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SET
0b // len
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
09 // len
06 // oid->tag MBEDTLS_ASN1_OID
03 // oid->len
55 04 06 // oid->p
13 // val->tag
02 // val->len
41 55 // val->p AU

31 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SET
13 // len
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
11 // len
06 // oid->tag
03 // oid->len
55 04 08 // oid->p
0c // val->tag
0a // val->len
53 6f 6d 65 2d 53 74 61 74 65 // val->p Some-State

31 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SET
21 // len
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
1f // len
06 // oid->tag MBEDTLS_ASN1_OID
03 // oid->len
55 04 0a // oid->p
0c // val->tag
18 // val->len
49 6e 74 65 72 6e 65 74 20 57
69 64 67 69 74 73 20 50 74 79
20 4c 74 64 // val->p Internet_Widgits_Pty_Ltd
## crt->subject_raw.p end

# SubjectPublicKeyInfo
## crt->pk_raw.p begin
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
82 // len: 0x80 | 0x02
01 22 // len
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
0d // len
06 // alg_oid->tag
09 // alg_oid->len
2a 86 48 86 f7 0d 01 01 01 // alg_oid->p oid_pk_alg
        // 2a MBEDTLS_OID_ISO_MEMBER_BODIES
        // 86 48 MBEDTLS_OID_COUNTRY_US
        // 86 f7 0d MBEDTLS_OID_ORG_RSA_DATA_SECURITY
        // MBEDTLS_OID_RSA_COMPANY
        // 01 MBEDTLS_OID_PKCS
        // 01 MBEDTLS_OID_PKCS1
        // 01 MBEDTLS_OID_PKCS1_RSA
05 // alg_params->tag
00 // alg_params->len

03 // tag: MBEDTLS_ASN1_BIT_STRING
82 // len: 0x80 | 0x02
01 0f // len
00 // reserved?
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
82 // len: 0x80 |  0x02
01 0a // len
02 // tag: MBEDTLS_ASN1_INTEGER
82 // len: 0x80 | 0x02
01 01 // len
## N begin
00 ef 83 a1 3e 3f 55 09 a4 f3
3d d8 31 3a fc 96 cb 34 75 d3
78 20 59 05 ac 09 17 aa 97 ef
ab 98 07 ba 26 c1 6d 24 6d e1
df 11 06 3c dc c2 30 65 b6 d1
2c 0b 87 11 6c 68 13 cb 47 05
21 33 8c e0 3e 97 40 54 d0 34
05 51 eb 44 15 3e ae fa 8e 67
2f 16 1d 32 3b e4 c5 df 4e 2b
37 68 a1 1c 21 c8 b1 04 4f 67

77 29 95 f6 a9 84 a2 e6 33 8e
3b 6b 3d cf 7a d4 a6 6c e9 38
ff ef c0 e1 1a f8 f2 3c 9c 9d
d3 2d 94 88 d9 e2 fa c4 94 b7
51 6b 75 c5 1f 86 a9 f6 35 d4
19 e7 52 80 e2 3a 5e 6d ce 63
4e f2 7c 2c f7 16 24 ac 7e d9
cf 68 05 58 15 67 03 e9 db 07
9b b0 2e 2f 65 cd 26 83 c5 d2
b6 55 14 33 e6 02 23 19 0e fe

46 6e 6e 6e 6b eb b1 6f b0 30
dc e2 07 31 75 c1 9f d4 ea 41
26 40 64 4e 8a 9f 43 cf c4 73
b1 0e 8b 32 5e 25 62 4f 14 79
87 5b b3 dc 79 0b eb ae 3f 69

be d3 e1 a8 89 1c 35
## N end

02 // tag: MBEDTLS_ASN1_INTEGER
03 // len
## E begin
01 00 01
## E end
## crt->pk_raw.p end

# issuerUniqueID
a3 // tag: MBEDTLS_ASN1_CONTEXT_SPECIFIC | MBEDTLS_ASN1_CONSTRUCTED | 3
53 // len
## ext->p
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
51 // len

## MBEDTLS_X509_EXT_SUBJECT_KEY_IDENTIFIER
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
1d // len
06 // tag: MBEDTLS_ASN1_OID extn_oid.tag
03 // extn_oid.len
55 1d 0e // extn_oid.p
        // 55 MBEDTLS_OID_ISO_CCITT_DS
        // 1d MBEDTLS_OID_ID_CE
        // 0e MBEDTLS_OID_SUBJECT_KEY_IDENTIFIER
04 // tag: MBEDTLS_ASN1_OCTET_STRING
16 // len
04 // tag: MBEDTLS_ASN1_OCTET_STRING
14 // len
37 b1 eb 78 38 69 66 53 b8 a1
57 ec 09 4a c4 91 56 ec 55 3b

## MBEDTLS_X509_EXT_AUTHORITY_KEY_IDENTIFIER
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
1f // len
06 // tag: MBEDTLS_ASN1_OID extn_oid.tag
03 // extn_oid.len
55 1d 23 // extn_oid.p
        // 55 MBEDTLS_OID_ISO_CCITT_DS
        // 1d MBEDTLS_OID_ID_CE
        // 23 MBEDTLS_OID_AUTHORITY_KEY_IDENTIFIER
04 // tag: MBEDTLS_ASN1_OCTET_STRING
18 // len
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
16 // len
80 // tag: MBEDTLS_ASN1_CONTEXT_SPECIFIC
14 // len
37 b1 eb 78 38 69 66 53 b8 a1
57 ec 09 4a c4 91 56 ec 55 3b

## MBEDTLS_X509_EXT_BASIC_CONSTRAINTS
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
0f // len
06 // tag: MBEDTLS_ASN1_OID extn_oid.tag
03 // extn_oid.len
55 1d 13 // extn_oid.p
        // 55 MBEDTLS_OID_ISO_CCITT_DS
        // 1d MBEDTLS_OID_ID_CE
        // 13 MBEDTLS_OID_BASIC_CONSTRAINTS
01 // tag: MBEDTLS_ASN1_BOOLEAN
01 // len
ff // != 0 ? 1: 0
04 // tag: MBEDTLS_ASN1_OCTET_STRING
05 // len
30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
03 // len
01 // tag: MBEDTLS_ASN1_BOOLEAN
01 // len
ff // != 0 ? 1 : 0

# end of TBSCertificate

30 // tag: MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE
0d // len
06 // tag: MBEDTLS_ASN1_OID alg->tag
09 // alg->len
2a 86 48 86 f7 0d 01 01 0b // alg->p
05 // params->tag
00 // params->len
03 // tag: MBEDTLS_ASN1_BIT_STRING crt->sig.tag
82 // len: 0x80 | 0x02
01 01 // len crt->sig.len
00 // reserved?
95 e6 08 8c c2 0c 01 4d f2 ba
b1 0f 0f b3 70 8c 0a 01 c9 e3
79 5e 95 4d 27 58 45 f2 46 47
30 a0 81 ad 9e a7 c4 ae 7f c6
fe fc 20 13 af b1 8a ba 9b ec
3d 50 a8 b2 15 5a 05 71 f7 eb
91 5e 63 24 6d 0d e9 43 67 7d
98 d9 65 4b 48 48 bc 26 e8 3c
c6 40 45 e2 f5 c9 8c 35 c9 9c
47 be d0 b0 92 9e 0d 46 0f 41

69 6a 56 5c d0 38 95 67 a1 6d
52 2b e2 3b bd 37 d0 83 6d 00
a1 49 ef 65 0a b5 e7 f1 64 48
38 33 2c 0a 2c 91 6b 22 a8 12
73 24 d6 62 b0 97 3c 3b 73 64
e1 0f 5f 27 15 f7 86 ac 3a b8
a2 2f 57 07 a3 7c e2 fd fb c6
a7 49 ec 3e 24 d1 1b 07 c3 e7
fd c5 32 cc 7a 78 f5 15 4d bd
c1 cf e8 3b 27 69 26 75 0c 09

29 a6 56 66 d9 13 31 3d 9c b8
04 dc b0 4a be 05 f3 4b c7 cf
a7 8e eb f0 42 73 6e b9 78 ef
7d 14 49 71 ca a8 b9 05 b1 07
c6 c0 7f 83 5b 9f 2b 2d ec ae
7e a1 6d 4a 69 c6