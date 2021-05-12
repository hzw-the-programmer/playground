MS_Code\MS_MMI\source\mmi_app\app\browser\control\src\brw_utility.c
BRWUtil_DecodeGzip

```
uint8 *dest_buf = NULL;

IMGGZIP_DECODE_SRC_PARAM_T src_param = {0};
IMGGZIP_DECODE_INFO_PARAM_T dec_info = {0};
IMGGZIP_DECODE_INPUT_PARAM_T decode_input = {0};
IMGGZIP_DECODE_OUTPUT_T decode_output = {0};
IMGGZIP_DECODE_RESULT_E decode_result = IMGGZIP_DECODE_SUCCESS;

uint8 *str;
int len;

src_param.src_buf_ptr = gzip_data;
src_param.src_size = sizeof(gzip_data);
IMGGZIP_Get_Info(&src_param, &dec_info);
dest_buf = SCI_ALLOC_APP(dec_info.src_img_size);

gzip_initialize(0, 0);

decode_input.dst_buf_ptr = dest_buf;
decode_input.dst_buf_size = dec_info.src_img_size;
decode_input.src_buf_ptr = gzip_data;
decode_input.src_size = sizeof(gzip_data);
decode_result = IMGGZIP_Decode(&decode_input,&decode_output);

str_en(&str, &len);

SCI_ASSERT(len == dec_info.src_img_size);
SCI_ASSERT(SCI_MEMCMP(str, dest_buf, len) == 0);

gzip_release();
```
