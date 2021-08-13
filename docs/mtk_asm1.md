plutommi\Service\PhbSrv\PhbMemSrv.c
```
/* Use U32 to be 4-byte aligned */
static U32 srv_phb_mem_pool[MMI_PHB_MEM_SIZE / 4];
srv_phb_mem_id = kal_adm_create(srv_phb_mem_pool, sizeof(srv_phb_mem_pool), NULL, KAL_FALSE);
ptr = (kal_uint32*) kal_adm_alloc(srv_phb_mem_id, (U32)size);
kal_adm_free(srv_phb_mem_id, memblock);

kal_adm_print_log(srv_phb_mem_id);
corrupted_mem_address = kal_adm_check_integrity(srv_phb_mem_id);
leftSize = (S32)kal_adm_get_total_left_size(srv_phb_mem_id);
```

plutommi\mmi\Resource\MemoryRes.c
```
ALIGN(ASM_ALIGN_SIZE)  U8 g_applib_mem_ap_pool[APPLIB_MEM_AP_POOL_SIZE + 10240];
```

plutommi\Framework\MemManager\MemManagerSrc\AppMemMgrCore.c
```
mmi_frm_appmem_stage1_init
```

applib\mem\src\app_mem.c
```
g_applib_mem_cntx.app_pool_id = (kal_uint32) kal_adm_create(
                                        mem_pool,
                                        pool_size,
                                        (kal_uint32*) g_applib_mem_pool_chunk_size,
                                        KAL_FALSE);
result = kal_adm_get_max_alloc_size((KAL_ADM_ID)g_applib_mem_cntx.app_pool_id);
chunk = kal_adm_alloc_nc_align(
                        (KAL_ADM_ID)g_applib_mem_cntx.app_pool_id,
                        chunk_size,
                        __MMI_GDI_LAYER_FRAMEBUF_ALIGNMENT__);
```

```
MoDIS.exe!applib_mem_ap_init(void (unsigned int, unsigned int, kal_bool)* stop_finish_callback_by_MMI=0x00e05c40, void * mem_pool=0x09ebfe38, unsigned int pool_size=3594885)  行2420	C
MoDIS.exe!mmi_frm_appmem_stage1_init()  行3459 + 0x16 字节	C
MoDIS.exe!MMI_Init(task_indx_type task_indx=INDX_MMI)  行2209	C
MoDIS.exe!_stack_init_tasks()  + 0x6c 字节	
MoDIS.exe!_stack_init()  + 0x82 字节	
MoDIS.exe!_mainp()  + 0xd 字节	
MoDIS.exe!MtkWinMainStart(void * argv=0x00000000)  行1144	C
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0cd26bc0)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!mmi_frm_appmem_init()  行3494	C
MoDIS.exe!initialize_UI_demo()  行3066	C
MoDIS.exe!initialize_UI_demo_adp(_mmi_event_struct * evt=0x0fc7fd44)  行1174 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc7fd44, const mmi_bootup_init_table_entry_struct * table=0x024f7fe0)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_early_init()  行221 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_early_init(void * arg=0x00000000, const mmi_frm_proc_id_info_struct id_info={...})  行505	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=10001, int (_mmi_event_struct *)* completed_proc=0x01a17200, void * user_data=0x0a26c2e8, const mmi_frm_proc_struct*child=0x024f7e14)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_seq_continue_execute(mmi_frm_proc_seq_cntx_struct * cntx=0x0a26c2e8)  行546 + 0x19 字节	C
MoDIS.exe!mmi_frm_proc_seq_entry(void * user_data=0x0a26c2e8, const mmi_frm_proc_id_info_struct id_info={...})  行596 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a34600, void * user_data=0x0a26890c, const mmi_frm_proc_struct*child=0x0a26c2dc)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_post_complete_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a34600, void * user_data=0x0a26890c,constmmi_frm_proc_struct * child=0x0a26c2dc)  行465 + 0x16 字节	C
MoDIS.exe!mmi_bootup_flow_start(_mmi_event_struct * evt=0x0fc7feb8)  行342 + 0x18 字节	C
MoDIS.exe!execute_evt_cb(unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc7feb8)  行434 + 0x14 字节	C
MoDIS.exe!emit_evt_cb(unsigned short evt_id=7956, unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc7feb8, int (_mmi_event_struct *)* call_back=0x00000000, void*user_data=0x00000000)  行572 + 0xd 字节	C
MoDIS.exe!process_cb_table(unsigned short evt_id=7956, int (unsigned short, unsigned int, _mmi_event_struct *, int (_mmi_event_struct *)*, void *)* cb_proc=0x00dfd8d0_mmi_event_struct * evt=0x0fc7feb8, int (_mmi_event_struct *)* call_back=0x00000000, void * user_data=0x00000000)  行357 + 0x1a 字节	C
MoDIS.exe!mmi_frm_cb_emit_event(_mmi_event_struct * evt=0x0fc7feb8)  行777 + 0x19 字节	C
MoDIS.exe!srv_bootup_power_on_ind_hdlr(void * msg=0x0a26bcc8)  行296 + 0x9 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a290fac)  行2694 + 0x9 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb44280)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5366, unsigned int app_string_id=38934, unsigned int app_icon_id=38903, void (void)* stop_callback_by_MMI=0x00cc8a5c)  行2641	C
MoDIS.exe!mmi_uc_asm_register_app()  行2163 + 0x19 字节	C
MoDIS.exe!mmi_uc_init()  行368	C
MoDIS.exe!mmi_uc_init_adp(_mmi_event_struct * evt=0x0fc8fd44)  行1129 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fd44, const mmi_bootup_init_table_entry_struct * table=0x024f7fe0)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_early_init()  行221 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_early_init(void * arg=0x00000000, const mmi_frm_proc_id_info_struct id_info={...})  行505	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=10001, int (_mmi_event_struct *)* completed_proc=0x01a175f0, void * user_data=0x0a26c4a8, const mmi_frm_proc_struc* child=0x024f7e14)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_seq_continue_execute(mmi_frm_proc_seq_cntx_struct * cntx=0x0a26c4a8)  行546 + 0x19 字节	C
MoDIS.exe!mmi_frm_proc_seq_entry(void * user_data=0x0a26c4a8, const mmi_frm_proc_id_info_struct id_info={...})  行596 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a349f0, void * user_data=0x0a268acc, const mmi_frm_proc_struct child=0x0a26c49c)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_post_complete_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a349f0, void * user_data=0x0a268acc, consmmi_frm_proc_struct * child=0x0a26c49c)  行465 + 0x16 字节	C
MoDIS.exe!mmi_bootup_flow_start(_mmi_event_struct * evt=0x0fc8feb8)  行342 + 0x18 字节	C
MoDIS.exe!execute_evt_cb(unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc8feb8)  行434 + 0x14 字节	C
MoDIS.exe!emit_evt_cb(unsigned short evt_id=7956, unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc8feb8, int (_mmi_event_struct *)* call_back=0x00000000, void user_data=0x00000000)  行572 + 0xd 字节	C
MoDIS.exe!process_cb_table(unsigned short evt_id=7956, int (unsigned short, unsigned int, _mmi_event_struct *, int (_mmi_event_struct *)*, void *)* cb_proc=0x00dfd8d0_mmi_event_struct * evt=0x0fc8feb8, int (_mmi_event_struct *)* call_back=0x00000000, void * user_data=0x00000000)  行357 + 0x1a 字节	C
MoDIS.exe!mmi_frm_cb_emit_event(_mmi_event_struct * evt=0x0fc8feb8)  行777 + 0x19 字节	C
MoDIS.exe!srv_bootup_power_on_ind_hdlr(void * msg=0x0a26bf78)  行296 + 0x9 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2694 + 0x9 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5412, unsigned int app_string_id=11957, unsigned int app_icon_id=11949, void (void)* stop_callback_by_MMI=0x00fe84a0)  行2641	C
MoDIS.exe!mmi_em_comm_mem_ap_register()  行206 + 0x19 字节	C
MoDIS.exe!mmi_em_init_asm_mem()  行823	C
MoDIS.exe!InitEngineerMode()  行1154	C
MoDIS.exe!InitEngineerMode_adp(_mmi_event_struct * evt=0x0fc8fd44)  行1193 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fd44, const mmi_bootup_init_table_entry_struct * table=0x024f7fe0)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_early_init()  行221 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_early_init(void * arg=0x00000000, const mmi_frm_proc_id_info_struct id_info={...})  行505	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=10001, int (_mmi_event_struct *)* completed_proc=0x01a175f0, void * user_data=0x0a26c4a8, const mmi_frm_proc_struc* child=0x024f7e14)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_seq_continue_execute(mmi_frm_proc_seq_cntx_struct * cntx=0x0a26c4a8)  行546 + 0x19 字节	C
MoDIS.exe!mmi_frm_proc_seq_entry(void * user_data=0x0a26c4a8, const mmi_frm_proc_id_info_struct id_info={...})  行596 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a349f0, void * user_data=0x0a268acc, const mmi_frm_proc_struct child=0x0a26c49c)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_post_complete_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a349f0, void * user_data=0x0a268acc, consmmi_frm_proc_struct * child=0x0a26c49c)  行465 + 0x16 字节	C
MoDIS.exe!mmi_bootup_flow_start(_mmi_event_struct * evt=0x0fc8feb8)  行342 + 0x18 字节	C
MoDIS.exe!execute_evt_cb(unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc8feb8)  行434 + 0x14 字节	C
MoDIS.exe!emit_evt_cb(unsigned short evt_id=7956, unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc8feb8, int (_mmi_event_struct *)* call_back=0x00000000, void user_data=0x00000000)  行572 + 0xd 字节	C
MoDIS.exe!process_cb_table(unsigned short evt_id=7956, int (unsigned short, unsigned int, _mmi_event_struct *, int (_mmi_event_struct *)*, void *)* cb_proc=0x00dfd8d0_mmi_event_struct * evt=0x0fc8feb8, int (_mmi_event_struct *)* call_back=0x00000000, void * user_data=0x00000000)  行357 + 0x1a 字节	C
MoDIS.exe!mmi_frm_cb_emit_event(_mmi_event_struct * evt=0x0fc8feb8)  行777 + 0x19 字节	C
MoDIS.exe!srv_bootup_power_on_ind_hdlr(void * msg=0x0a26bf78)  行296 + 0x9 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2694 + 0x9 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5413, unsigned int app_string_id=13492, unsigned int app_icon_id=0, void (void)* stop_callback_by_MMI=0x00fdd150)  行2641	C
MoDIS.exe!InitFactoryMode()  行2116 + 0x16 字节	C
MoDIS.exe!InitFactoryMode_adp(_mmi_event_struct * evt=0x0fc8fd44)  行1196 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fd44, const mmi_bootup_init_table_entry_struct * table=0x024f7fe0)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_early_init()  行221 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_early_init(void * arg=0x00000000, const mmi_frm_proc_id_info_struct id_info={...})  行505	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=10001, int (_mmi_event_struct *)* completed_proc=0x01a175f0, void * user_data=0x0a26c4a8, const mmi_frm_proc_struct* child=0x024f7e14)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_seq_continue_execute(mmi_frm_proc_seq_cntx_struct * cntx=0x0a26c4a8)  行546 + 0x19 字节	C
MoDIS.exe!mmi_frm_proc_seq_entry(void * user_data=0x0a26c4a8, const mmi_frm_proc_id_info_struct id_info={...})  行596 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a349f0, void * user_data=0x0a268acc, const mmi_frm_proc_struct *child=0x0a26c49c)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_post_complete_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a349f0, void * user_data=0x0a268acc, constmmi_frm_proc_struct * child=0x0a26c49c)  行465 + 0x16 字节	C
MoDIS.exe!mmi_bootup_flow_start(_mmi_event_struct * evt=0x0fc8feb8)  行342 + 0x18 字节	C
MoDIS.exe!execute_evt_cb(unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc8feb8)  行434 + 0x14 字节	C
MoDIS.exe!emit_evt_cb(unsigned short evt_id=7956, unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc8feb8, int (_mmi_event_struct *)* call_back=0x00000000, void *user_data=0x00000000)  行572 + 0xd 字节	C
MoDIS.exe!process_cb_table(unsigned short evt_id=7956, int (unsigned short, unsigned int, _mmi_event_struct *, int (_mmi_event_struct *)*, void *)* cb_proc=0x00dfd8d0,_mmi_event_struct * evt=0x0fc8feb8, int (_mmi_event_struct *)* call_back=0x00000000, void * user_data=0x00000000)  行357 + 0x1a 字节	C
MoDIS.exe!mmi_frm_cb_emit_event(_mmi_event_struct * evt=0x0fc8feb8)  行777 + 0x19 字节	C
MoDIS.exe!srv_bootup_power_on_ind_hdlr(void * msg=0x0a26bf78)  行296 + 0x9 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2694 + 0x9 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5382, unsigned int app_string_id=34248, unsigned int app_icon_id=0, void (void)* stop_callback_by_MMI=0x01288250)  行2641	C
MoDIS.exe!mmi_oppc_init()  行2676 + 0x25 字节	C
MoDIS.exe!mmi_bt_opp_init()  行694	C
MoDIS.exe!mmi_bt_init()  行1571 + 0xe 字节	C
MoDIS.exe!mmi_bt_init_adp(_mmi_event_struct * evt=0x0fc8fd44)  行1232 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fd44, const mmi_bootup_init_table_entry_struct * table=0x024f7fe0)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_early_init()  行221 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_early_init(void * arg=0x00000000, const mmi_frm_proc_id_info_struct id_info={...})  行505	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=10001, int (_mmi_event_struct *)* completed_proc=0x01a175f0, void * user_data=0x0a26c4a8, const mmi_frm_proc_struct* child=0x024f7e14)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_seq_continue_execute(mmi_frm_proc_seq_cntx_struct * cntx=0x0a26c4a8)  行546 + 0x19 字节	C
MoDIS.exe!mmi_frm_proc_seq_entry(void * user_data=0x0a26c4a8, const mmi_frm_proc_id_info_struct id_info={...})  行596 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a349f0, void * user_data=0x0a268acc, const mmi_frm_proc_struct *child=0x0a26c49c)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_post_complete_execute(unsigned short parent_id=1, int (_mmi_event_struct *)* completed_proc=0x01a349f0, void * user_data=0x0a268acc, constmmi_frm_proc_struct * child=0x0a26c49c)  行465 + 0x16 字节	C
MoDIS.exe!mmi_bootup_flow_start(_mmi_event_struct * evt=0x0fc8feb8)  行342 + 0x18 字节	C
MoDIS.exe!execute_evt_cb(unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc8feb8)  行434 + 0x14 字节	C
MoDIS.exe!emit_evt_cb(unsigned short evt_id=7956, unsigned int tbl_idx=234, _mmi_event_struct * evt=0x0fc8feb8, int (_mmi_event_struct *)* call_back=0x00000000, void *user_data=0x00000000)  行572 + 0xd 字节	C
MoDIS.exe!process_cb_table(unsigned short evt_id=7956, int (unsigned short, unsigned int, _mmi_event_struct *, int (_mmi_event_struct *)*, void *)* cb_proc=0x00dfd8d0,_mmi_event_struct * evt=0x0fc8feb8, int (_mmi_event_struct *)* call_back=0x00000000, void * user_data=0x00000000)  行357 + 0x1a 字节	C
MoDIS.exe!mmi_frm_cb_emit_event(_mmi_event_struct * evt=0x0fc8feb8)  行777 + 0x19 字节	C
MoDIS.exe!srv_bootup_power_on_ind_hdlr(void * msg=0x0a26bf78)  行296 + 0x9 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2694 + 0x9 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5416, unsigned int app_string_id=41156, unsigned int app_icon_id=0, void (void)* stop_callback_by_MMI=0x00ef1cc0)  行2641	C
MoDIS.exe!mmi_prof_app_init(_mmi_event_struct * evt=0x0fc8fcd0)  行1612 + 0x25 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fcd0, const mmi_bootup_init_table_entry_struct * table=0x024f7f08)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_normal_init(void * arg=0x00000000, const mmi_frm_proc_id_info_struct id_info={...})  行262 + 0xd 字节	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=10003, int (_mmi_event_struct *)* completed_proc=0x01a17be0, void * user_data=0x0a26c1d8, const mmi_frm_proc_struct* child=0x024f7de8)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_con_entry(void * user_data=0x0a26c1d8, const mmi_frm_proc_id_info_struct id_info={...})  行919 + 0x19 字节	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=11, int (_mmi_event_struct *)* completed_proc=0x01a17f50, void * user_data=0x0a26b4c8, const mmi_frm_proc_struct *child=0x0a26c1cc)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_con_create_and_execute(void * arg=0x024f7df4, const mmi_frm_proc_id_info_struct id_info={...})  行1170 + 0x19 字节	C
MoDIS.exe!mmi_frm_proc_execute(unsigned short parent_id=10001, int (_mmi_event_struct *)* completed_proc=0x01a175f0, void * user_data=0x0a26c4a8, const mmi_frm_proc_struct* child=0x024f7e44)  行418 + 0x15 字节	C
MoDIS.exe!mmi_frm_proc_seq_continue_execute(mmi_frm_proc_seq_cntx_struct * cntx=0x0a26c4a8)  行546 + 0x19 字节	C
MoDIS.exe!mmi_frm_proc_seq_child_completed_proc(_mmi_event_struct * evt=0x0fc8fe28)  行674 + 0x9 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fe28, int (_mmi_event_struct *)* proc=0x01a175f0, void * user_data=0x0a26c4a8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_bootup_on_sim_module_ready(void * user_data=0x0a26927c)  行321 + 0xb 字节	C
MoDIS.exe!srv_bootup_init_sim_callback_proc(_mmi_event_struct * evt=0x03d6a228)  行364 + 0x9 字节	C
MoDIS.exe!mmi_frm_invoke_post_event()  行1537 + 0x10 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2722	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5386, unsigned int app_string_id=41387, unsigned int app_icon_id=50108, void (void)* stop_callback_by_MMI=0x00cc565e)  行2641	C
MoDIS.exe!mmi_fmgri_main_init_mmi_context()  行2476 + 0x25 字节	C
MoDIS.exe!mmi_fmgr_init_mmi()  行702	C
MoDIS.exe!mmi_fmgr_init_mmi_adp(_mmi_event_struct * evt=0x0fc8fcc8)  行923 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fcc8, const mmi_bootup_init_table_entry_struct * table=0x024f7f48)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_notify_before_idle()  行299 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_completed(_mmi_event_struct * evt=0x0fc8fd08)  行847	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd08, int (_mmi_event_struct *)* proc=0x01a349f0, void * user_data=0x0a268acc)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_seq_child_completed_proc(_mmi_event_struct * evt=0x0fc8fd80)  行682 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd80, int (_mmi_event_struct *)* proc=0x01a175f0, void * user_data=0x0a26c4a8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_cr_ex_child_completed_proc(_mmi_event_struct * evt=0x0fc8fdcc)  行1208 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fdcc, int (_mmi_event_struct *)* proc=0x01a17f50, void * user_data=0x0a26b4c8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_child_completed_proc(_mmi_event_struct * evt=0x0fc8fe24)  行994 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fe24, int (_mmi_event_struct *)* proc=0x01a17be0, void * user_data=0x0a26c1d8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_bootup_on_protocol_ready(void * user_data=0x00000000)  行950 + 0xb 字节	C
MoDIS.exe!srv_bootup_protocol_l4_response_hdlr(void * msg=0x0a26a658, int src_mod=26)  行935 + 0xf 字节	C
MoDIS.exe!mmi_frm_execute_current_protocol_handler(unsigned short eventID=14853, void * MsgStruct=0x0a26a658, int mod_src=26, void * Message=0x0fc8fefc)  行670 + 0x11 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2711 + 0x16 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5390, unsigned int app_string_id=46995, unsigned int app_icon_id=46995, void (void)* stop_callback_by_MMI=0x00ffe1d0)  行2641	C
MoDIS.exe!mmi_medply_init_app()  行17706 + 0x19 字节	C
MoDIS.exe!mmi_fng_init_app()  行1202	C
MoDIS.exe!mmi_fng_init_app_adp(_mmi_event_struct * evt=0x0fc8fcc8)  行926 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fcc8, const mmi_bootup_init_table_entry_struct * table=0x024f7f48)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_notify_before_idle()  行299 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_completed(_mmi_event_struct * evt=0x0fc8fd08)  行847	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd08, int (_mmi_event_struct *)* proc=0x01a349f0, void * user_data=0x0a268acc)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_seq_child_completed_proc(_mmi_event_struct * evt=0x0fc8fd80)  行682 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd80, int (_mmi_event_struct *)* proc=0x01a175f0, void * user_data=0x0a26c4a8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_cr_ex_child_completed_proc(_mmi_event_struct * evt=0x0fc8fdcc)  行1208 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fdcc, int (_mmi_event_struct *)* proc=0x01a17f50, void * user_data=0x0a26b4c8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_child_completed_proc(_mmi_event_struct * evt=0x0fc8fe24)  行994 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fe24, int (_mmi_event_struct *)* proc=0x01a17be0, void * user_data=0x0a26c1d8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_bootup_on_protocol_ready(void * user_data=0x00000000)  行950 + 0xb 字节	C
MoDIS.exe!srv_bootup_protocol_l4_response_hdlr(void * msg=0x0a26a658, int src_mod=26)  行935 + 0xf 字节	C
MoDIS.exe!mmi_frm_execute_current_protocol_handler(unsigned short eventID=14853, void * MsgStruct=0x0a26a658, int mod_src=26, void * Message=0x0fc8fefc)  行670 + 0x11 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2711 + 0x16 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5381, unsigned int app_string_id=37365, unsigned int app_icon_id=37365, void (void)* stop_callback_by_MMI=0x00000000)  行2641	C
MoDIS.exe!mmi_camco_init_app()  行9462 + 0x16 字节	C
MoDIS.exe!mmi_fng_init_app()  行1210	C
MoDIS.exe!mmi_fng_init_app_adp(_mmi_event_struct * evt=0x0fc8fcc8)  行926 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fcc8, const mmi_bootup_init_table_entry_struct * table=0x024f7f48)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_notify_before_idle()  行299 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_completed(_mmi_event_struct * evt=0x0fc8fd08)  行847	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd08, int (_mmi_event_struct *)* proc=0x01a349f0, void * user_data=0x0a268acc)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_seq_child_completed_proc(_mmi_event_struct * evt=0x0fc8fd80)  行682 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd80, int (_mmi_event_struct *)* proc=0x01a175f0, void * user_data=0x0a26c4a8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_cr_ex_child_completed_proc(_mmi_event_struct * evt=0x0fc8fdcc)  行1208 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fdcc, int (_mmi_event_struct *)* proc=0x01a17f50, void * user_data=0x0a26b4c8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_child_completed_proc(_mmi_event_struct * evt=0x0fc8fe24)  行994 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fe24, int (_mmi_event_struct *)* proc=0x01a17be0, void * user_data=0x0a26c1d8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_bootup_on_protocol_ready(void * user_data=0x00000000)  行950 + 0xb 字节	C
MoDIS.exe!srv_bootup_protocol_l4_response_hdlr(void * msg=0x0a26a658, int src_mod=26)  行935 + 0xf 字节	C
MoDIS.exe!mmi_frm_execute_current_protocol_handler(unsigned short eventID=14853, void * MsgStruct=0x0a26a658, int mod_src=26, void * Message=0x0fc8fefc)  行670 + 0x11 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2711 + 0x16 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5408, unsigned int app_string_id=35702, unsigned int app_icon_id=35702, void (void)* stop_callback_by_MMI=0x01078bc0)  行2641	C
MoDIS.exe!mmi_ivex_init_app()  行2072 + 0x19 字节	C
MoDIS.exe!mmi_imgview_init_app()  行2817	C
MoDIS.exe!mmi_fng_init_app()  行1227	C
MoDIS.exe!mmi_fng_init_app_adp(_mmi_event_struct * evt=0x0fc8fcc8)  行926 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fcc8, const mmi_bootup_init_table_entry_struct * table=0x024f7f48)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_notify_before_idle()  行299 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_completed(_mmi_event_struct * evt=0x0fc8fd08)  行847	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd08, int (_mmi_event_struct *)* proc=0x01a349f0, void * user_data=0x0a268acc)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_seq_child_completed_proc(_mmi_event_struct * evt=0x0fc8fd80)  行682 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd80, int (_mmi_event_struct *)* proc=0x01a175f0, void * user_data=0x0a26c4a8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_cr_ex_child_completed_proc(_mmi_event_struct * evt=0x0fc8fdcc)  行1208 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fdcc, int (_mmi_event_struct *)* proc=0x01a17f50, void * user_data=0x0a26b4c8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_child_completed_proc(_mmi_event_struct * evt=0x0fc8fe24)  行994 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fe24, int (_mmi_event_struct *)* proc=0x01a17be0, void * user_data=0x0a26c1d8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_bootup_on_protocol_ready(void * user_data=0x00000000)  行950 + 0xb 字节	C
MoDIS.exe!srv_bootup_protocol_l4_response_hdlr(void * msg=0x0a26a658, int src_mod=26)  行935 + 0xf 字节	C
MoDIS.exe!mmi_frm_execute_current_protocol_handler(unsigned short eventID=14853, void * MsgStruct=0x0a26a658, int mod_src=26, void * Message=0x0fc8fefc)  行670 + 0x11 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2711 + 0x16 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!applib_mem_ap_register(unsigned int app_id=5364, unsigned int app_string_id=14444, unsigned int app_icon_id=14458, void (void)* stop_callback_by_MMI=0x00cd58ce)  行2641	C
MoDIS.exe!InitJAVA()  行3050 + 0x19 字节	C
MoDIS.exe!mmi_fng_init_app()  行1248	C
MoDIS.exe!mmi_fng_init_app_adp(_mmi_event_struct * evt=0x0fc8fcc8)  行926 + 0x8 字节	C
MoDIS.exe!mmi_bootup_init_apps(_mmi_event_struct * evt=0x0fc8fcc8, const mmi_bootup_init_table_entry_struct * table=0x024f7f48)  行182 + 0xd 字节	C
MoDIS.exe!mmi_bootup_notify_before_idle()  行299 + 0xd 字节	C
MoDIS.exe!mmi_bootup_flow_completed(_mmi_event_struct * evt=0x0fc8fd08)  行847	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd08, int (_mmi_event_struct *)* proc=0x01a349f0, void * user_data=0x0a268acc)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_seq_child_completed_proc(_mmi_event_struct * evt=0x0fc8fd80)  行682 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fd80, int (_mmi_event_struct *)* proc=0x01a175f0, void * user_data=0x0a26c4a8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_cr_ex_child_completed_proc(_mmi_event_struct * evt=0x0fc8fdcc)  行1208 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fdcc, int (_mmi_event_struct *)* proc=0x01a17f50, void * user_data=0x0a26b4c8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_frm_proc_con_child_completed_proc(_mmi_event_struct * evt=0x0fc8fe24)  行994 + 0xc 字节	C
MoDIS.exe!mmi_frm_send_event(_mmi_event_struct * evt=0x0fc8fe24, int (_mmi_event_struct *)* proc=0x01a17be0, void * user_data=0x0a26c1d8)  行1156 + 0x9 字节	C
MoDIS.exe!mmi_frm_proc_notify_completed(const mmi_frm_proc_id_info_struct id_info={...})  行328 + 0x17 字节	C
MoDIS.exe!mmi_bootup_on_protocol_ready(void * user_data=0x00000000)  行950 + 0xb 字节	C
MoDIS.exe!srv_bootup_protocol_l4_response_hdlr(void * msg=0x0a26a658, int src_mod=26)  行935 + 0xf 字节	C
MoDIS.exe!mmi_frm_execute_current_protocol_handler(unsigned short eventID=14853, void * MsgStruct=0x0a26a658, int mod_src=26, void * Message=0x0fc8fefc)  行670 + 0x11 字节	C
MoDIS.exe!MMI_task(task_entry_struct * entry_param=0x0a29116c)  行2711 + 0x16 字节	C
MoDIS.exe!_osc_platform_thread_create()  + 0x2a0 字节	
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0eb54268)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```
