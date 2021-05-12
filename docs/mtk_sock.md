/*
    模拟器上第一次block。
    真机上几乎每次都block。
*/
extern kal_int8 soc_gethostbyname(kal_bool is_blocking,
                           module_type     mod_id,
                           kal_int32       request_id,
                           const kal_char  *domain_name,
                           kal_uint8       *addr,
                           kal_uint8       *addr_len,
                           kal_uint8       access_id,
                           kal_uint32      nwk_account_id);
