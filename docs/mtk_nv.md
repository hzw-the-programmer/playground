custom\common\PLUTO_MMI\nvram_common_defs.h
__MMI_FM_RADIO_SCHEDULE_REC__

custom\common\PLUTO_MMI\nvram_common_defs.h
make\Gsm2.mak
```
if exist $(FIXPATH)\$(MMIDIR)\Customer\CustResource\$(strip $(CUSTOMER))_MMI\Nvram_common_config.c \
 		(copy /y $(FIXPATH)\$(MMIDIR)\Customer\CustResource\$(strip $(CUSTOMER))_MMI\Nvram_common_config.c $(FIXPATH)\custom\common\PLUTO_MMI\*.* > nul)
```
```
if exist $(FIXPATH)\$(MMIDIR)\Customer\CustResource\$(strip $(CUSTOMER))_MMI\common_nvram_editor_data_item.h \
 		(copy /y $(FIXPATH)\$(MMIDIR)\Customer\CustResource\$(strip $(CUSTOMER))_MMI\Common_nvram_editor_data_item.h $(FIXPATH)\custom\common\pluto_mmi\*.* > nul)
```
#make CUSTOMER gprs codegen
make CUSTOMER gprs resgen

nvram_ltable_construct

nvram_drv_fat_write

nvram_calculate_write_data_offset
nvram_create_package_file

```
MoDIS.exe!nvram_check_data_item(kal_bool sw_change=KAL_FALSE)  行591	C
MoDIS.exe!nvram_set_offset_to_ltable()  行705 + 0x7 字节	C
MoDIS.exe!nvram_pseudo_merge()  行1815 + 0x5 字节	C
MoDIS.exe!nvram_init(task_indx_type task_indx=INDX_NVRAM)  行2647 + 0x5 字节	C
MoDIS.exe!_stack_init_tasks()  + 0x6c 字节	
MoDIS.exe!_stack_init()  + 0x82 字节	
MoDIS.exe!_mainp()  + 0xd 字节	
MoDIS.exe!MtkWinMainStart(void * argv=0x00000000)  行1144	C
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0d466bc0)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!nvram_create_package_file(unsigned char M_name='A')  行435	C
MoDIS.exe!nvram_calculate_write_data_offset(unsigned int * offset_for_add_lid=0x00000000, kal_bool is_init=KAL_TRUE)  行792 + 0x7 字节	C
MoDIS.exe!nvram_reset_data_items(nvram_reset_category_enum reset_category=NVRAM_RESET_ALL, unsigned short app_id=0, nvram_ltable_entry_struct * ldi=0x00000000, short rec_index=0, unsigned short rec_amount=0)  行2111 + 0x9 字节	C
MoDIS.exe!nvram_init_all_ldi(nvram_reset_category_enum reset_category=NVRAM_RESET_ALL)  行2334 + 0x11 字节	C
MoDIS.exe!nvram_init(task_indx_type task_indx=INDX_NVRAM)  行2561 + 0x7 字节	C
MoDIS.exe!_stack_init_tasks()  + 0x6c 字节	
MoDIS.exe!_stack_init()  + 0x82 字节	
MoDIS.exe!_mainp()  + 0xd 字节	
MoDIS.exe!MtkWinMainStart(void * argv=0x00000000)  行1144	C
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0d1f6bc0)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	
```

```
MoDIS.exe!nvram_write_fs_data_item(nvram_ltable_entry_struct * ldi=0x0ae2c1a0, unsigned short index=1, unsigned char * buffer=0x02453914, unsigned int buffer_size=2, kabool is_init=KAL_TRUE)  行1775	C
MoDIS.exe!nvram_write_data_item(nvram_ltable_entry_struct * ldi=0x0ae2c1a0, unsigned short index=1, unsigned char * data=0x02453914, kal_bool is_init=KAL_TRUE)  行1613 0x1d 字节	C
MoDIS.exe!nvram_reset_one_data_item(nvram_ltable_entry_struct * ldi=0x0ae2c1a0, unsigned short rec_index=1, unsigned short rec_amount=1)  行1872 + 0x17 字节	C
MoDIS.exe!nvram_reset_category(unsigned int included=65535, unsigned int excluded=0)  行1953 + 0x13 字节	C
MoDIS.exe!nvram_reset_data_items(nvram_reset_category_enum reset_category=NVRAM_RESET_ALL, unsigned short app_id=0, nvram_ltable_entry_struct * ldi=0x00000000, unsigneshort rec_index=0, unsigned short rec_amount=0)  行2116 + 0xc 字节	C
MoDIS.exe!nvram_init_all_ldi(nvram_reset_category_enum reset_category=NVRAM_RESET_ALL)  行2334 + 0x11 字节	C
MoDIS.exe!nvram_init(task_indx_type task_indx=INDX_NVRAM)  行2561 + 0x7 字节	C
MoDIS.exe!_stack_init_tasks()  + 0x6c 字节	
MoDIS.exe!_stack_init()  + 0x82 字节	
MoDIS.exe!_mainp()  + 0xd 字节	
MoDIS.exe!MtkWinMainStart(void * argv=0x00000000)  行1144	C
msvcr90d.dll!_callthreadstartex()  行348 + 0xf 字节	C
msvcr90d.dll!_threadstartex(void * ptd=0x0cc96970)  行331	C
kernel32.dll!76c06359() 	
[下面的框架可能不正确和/或缺失，没有为 kernel32.dll 加载符号]	
ntdll.dll!770487a4() 	
ntdll.dll!77048774() 	

```

plutommi\Customer\CustResource\UL28_BTD_XB61_RIVO_S625_MMI\nvram_common_config.c
```
ltable_entry_struct logical_data_item_table_common_app[] =
{
	...
	{
        NVRAM_EF_ALM_ALARM_DATA_LID,
        NVRAM_ALM_ALARM_DATA_TOTAL,
        NVRAM_ALM_ALARM_DATA_SIZE ,
        NVRAM_NORMAL(NVRAM_EF_ZERO_DEFAULT),
        NVRAM_CATEGORY_USER,
        NVRAM_ATTR_SW_VERNO_RESET,
        "MP1N",
        VER(NVRAM_EF_ALM_ALARM_DATA_LID)
    },
	...
}
```

custom\common\PLUTO_MMI\nvram_common_defs.h
```
typedef enum
{
	...
	NVRAM_EF_ALM_ALARM_DATA_LID,
	...
} nvram_lid_commapp_enum;
```

custom\common\PLUTO_MMI\custom_mmi_default_value.h
```
typedef struct
{
    unsigned char Hour;
    ...
#ifdef __MMI_ALM_CUST_VOLUME__
    unsigned char Volume;
#endif
	...
    kal_uint32 timestamp;
} alm_nvram_struct;

#define NVRAM_ALM_ALARM_DATA_TOTAL  NUM_OF_ALM
#define NVRAM_ALM_ALARM_DATA_SIZE   sizeof(alm_nvram_struct)
```

plutommi\Customer\CustResource\UL28_BTD_XB61_RIVO_S625_MMI\common_nvram_editor_data_item.h
```
...
#define NVRAM_EF_ALM_ALARM_DATA_LID_VERNO               "000"
...
```
