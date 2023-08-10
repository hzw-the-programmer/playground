```
Compiling Source File version/PROJECT_project_barphone_version.c...

"C:\Program Files\ARM\Development Studio 2019.1\sw\ARMCompiler5.06u6/bin/armcc"
--cpu cortex-a7
-O2
--apcs /interwork/
--diag_suppress 1,9,61,66,68,111,117,144,
152,167,170,174,177,186,188,191,223,226,236,494,513,550,940,1134,1287,1294,1295,
1296,1381,1441,1608,1652,1764,1786,3017,3052
--fpu VFPv4
--enum_is_int
-D_RTOS
-D_DEBUG
--loose_implicit_cast
--li
--no_unaligned_access
-g
--thumb
--feedback build/project_240X320BAR_48MB_CAT1_builddir/project_240X320BAR_48MB_CAT1_feedback.txt
--via build/project_240X320BAR_48MB_CAT1_builddir/dep/version_C_MACRO_INC.txt
-c
-I"C:\Program Files\ARM\Development Studio 2019.1\sw\ARMCompiler5.06u6/include"
version/PROJECT_project_barphone_version.c
--depend build/project_240X320BAR_48MB_CAT1_builddir/dep/version/PROJECT_project_barphone_version.d.tmp
-o build/project_240X320BAR_48MB_CAT1_builddir/obj/version/PROJECT_project_barphone_version.o  \
```

```
Compiling Source File Third-party/picotls/lib/picotls.c...
File: build/project_240X320BAR_48MB_CAT1_builddir/lib/picotls.a delete OK!
File: build/project_240X320BAR_48MB_CAT1_builddir/obj/picotls/picotls.o delete OK!
"no source": Warning:  #3036-D: "C:\Program Files\DS-5 v5.22.0\sw\ARMCompiler5.05u2/include" was specified as both a system and non-system include directory --
the non-system entry will be ignored
"Third-party/picotls/lib/picotls.c", line 1511: Warning:  #1293-D: assignment in
 condition
      decode_extensions(src, end, PTLS_HANDSHAKE_TYPE_NEW_SESSION_TICKET, &extty
pe, {
      ^
"Third-party/picotls/lib/picotls.c", line 2558: Warning:  #1293-D: assignment in
 condition
      decode_extensions(src, end, sh->is_retry_request ? PTLS_HANDSHAKE_TYPE_PSE
UDO_HRR : PTLS_HANDSHAKE_TYPE_SERVER_HELLO, &exttype,
      ^
"Third-party/picotls/lib/picotls.c", line 2512: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
          goto Exit;
          ^
"Third-party/picotls/lib/picotls.c", line 2519: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
          goto Exit;
          ^
"Third-party/picotls/lib/picotls.c", line 2525: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
      ptls_decode_open_block(src, end, 1, {
      ^
"Third-party/picotls/lib/picotls.c", line 2525: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
      ptls_decode_open_block(src, end, 1, {
      ^
"Third-party/picotls/lib/picotls.c", line 2525: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
      ptls_decode_open_block(src, end, 1, {
      ^
"Third-party/picotls/lib/picotls.c", line 2525: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
      ptls_decode_open_block(src, end, 1, {
      ^
"Third-party/picotls/lib/picotls.c", line 2525: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
      ptls_decode_open_block(src, end, 1, {
      ^
"Third-party/picotls/lib/picotls.c", line 2537: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
              goto Exit;
              ^
"Third-party/picotls/lib/picotls.c", line 2540: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
              goto Exit;
              ^
"Third-party/picotls/lib/picotls.c", line 2547: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
              goto Exit;
              ^
"Third-party/picotls/lib/picotls.c", line 2550: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "found_version" (declared at line 2557)
            variable "selected_psk_identity" (declared at line 2557)
              goto Exit;
              ^
"Third-party/picotls/lib/picotls.c", line 2860: Warning:  #1293-D: assignment in
 condition
      decode_extensions(src, end, PTLS_HANDSHAKE_TYPE_ENCRYPTED_EXTENSIONS, &typ
e, {
      ^
"Third-party/picotls/lib/picotls.c", line 2995: Warning:  #1293-D: assignment in
 condition
      decode_extensions(src, end, PTLS_HANDSHAKE_TYPE_CERTIFICATE_REQUEST, &extt
ype, {
      ^
"Third-party/picotls/lib/picotls.c", line 3168: Warning:  #1293-D: assignment in
 condition
      ptls_decode_block(src, end, 3, {
      ^
"Third-party/picotls/lib/picotls.c", line 3579: Warning:  #1293-D: assignment in
 condition
      decode_extensions(src, end, PTLS_HANDSHAKE_TYPE_CLIENT_HELLO, &exttype, {
      ^
"Third-party/picotls/lib/picotls.c", line 5041: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "enc_secret" (declared at line 5047)
            variable "dec_secret" (declared at line 5048)
          goto Exit;
          ^
"Third-party/picotls/lib/picotls.c", line 5702: Warning:  #546-D: transfer of co
ntrol bypasses initialization of:
            variable "textlen" (declared at line 5708)
              goto Exit;
              ^
Third-party/picotls/lib/picotls.c: 20 warnings, 0 errors

picotls library build finished

picotls Time consuming: 5 seconds!
```