custom\common\PLUTO_MMI\nvram_common_defs.h
```
typedef enum
{
	NVRAM_EF_AUTOTEST_LID  = NVRAM_LID_COMMAPP_BEGIN,
	...
	NVRAM_EF_DEMO_APP_CONFIG_LID,
	NVRAM_EF_LAST_LID_COMMAPP
} nvram_lid_commapp_enum;
```

custom\common\PLUTO_MMI\custom_mmi_default_value.h
```
typedef struct
{
	kal_uint32 c1;
	kal_uint8  c2;
	kal_uint8  c3;
	kal_uint8  c4;
} demo_app_config;

#define NVRAM_EF_DEMO_APP_CONFIG_TOTAL 1
#define NVRAM_EF_DEMO_APP_CONFIG_SIZE  sizeof(demo_app_config)
```

plutommi\Customer\CustResource\{custom}\nvram_common_config.c
```
static demo_app_config const NVRAM_EF_DEMO_APP_CONFIG_DEFAULT[NVRAM_EF_DEMO_APP_CONFIG_TOTAL] =
{
	0x01020304, 0x03, 0x02, 0x01,
};

ltable_entry_struct logical_data_item_table_common_app[] =
{
	...
	{
		NVRAM_EF_DEMO_APP_CONFIG_LID,
		NVRAM_EF_DEMO_APP_CONFIG_TOTAL,
		NVRAM_EF_DEMO_APP_CONFIG_SIZE,
		NVRAM_NORMAL(NVRAM_EF_DEMO_APP_CONFIG_DEFAULT),
		NVRAM_CATEGORY_USER,
		NVRAM_ATTR_AVERAGE | NVRAM_ATTR_FACTORY_RESET, //NVRAM_ATTR_CONFIDENTIAL
		"MT90",
		VER(NVRAM_EF_DEMO_APP_CONFIG_LID)
	},
	...
}
```

plutommi\Customer\CustResource\{custom}\common_nvram_editor_data_item.h
```
#define NVRAM_EF_DEMO_APP_CONFIG_LID_VERNO                      "000"

LID_BIT VER_LID(NVRAM_EF_DEMO_APP_CONFIG_LID)
demo_app_config *NVRAM_EF_DEMO_APP_CONFIG_TOTAL
{
};

END_NVRAM_DATA
```

make {custom} gprs codegen

```
$ grep -r DEMO_APP custom/
custom/common/hal/nvram/custom_nvram_lid_cat.xml:    <NVRAMITEM id="NVRAM_EF_DEMO_APP_CONFIG_LID" prefix="MT90" version="000" record_size="8" total_record="1">
custom/common/hal/nvram/custom_nvram_lid_cat.xml:          <SYMBOL_NAME>NVRAM_EF_DEMO_APP_CONFIG_DEFAULT</SYMBOL_NAME>
custom/common/hal/nvram/custom_nvram_lid_cat.xml:        <DESCRIPTION> NVRAM_EF_DEMO_APP_CONFIG_LID </DESCRIPTION>
custom/common/hal/nvram/nvram_gen_trc.h:    _NVRAM_EF_DEMO_APP_CONFIG_LID = 401,
custom/common/PLUTO_MMI/common_nvram_editor_data_item.h:#define NVRAM_EF_DEMO_APP_CONFIG_LID_VERNO                      "000"
custom/common/PLUTO_MMI/common_nvram_editor_data_item.h:LID_BIT VER_LID(NVRAM_EF_DEMO_APP_CONFIG_LID)
custom/common/PLUTO_MMI/common_nvram_editor_data_item.h:demo_app_config *NVRAM_EF_DEMO_APP_CONFIG_TOTAL
custom/common/PLUTO_MMI/custom_mmi_default_value.h:#define NVRAM_EF_DEMO_APP_CONFIG_TOTAL 1
custom/common/PLUTO_MMI/custom_mmi_default_value.h:#define NVRAM_EF_DEMO_APP_CONFIG_SIZE  sizeof(demo_app_config)
custom/common/PLUTO_MMI/nvram_common_config.c:static demo_app_config const NVRAM_EF_DEMO_APP_CONFIG_DEFAULT[NVRAM_EF_DEMO_APP_CONFIG_TOTAL] =
custom/common/PLUTO_MMI/nvram_common_config.c:          NVRAM_EF_DEMO_APP_CONFIG_LID,
custom/common/PLUTO_MMI/nvram_common_config.c:          NVRAM_EF_DEMO_APP_CONFIG_TOTAL,
custom/common/PLUTO_MMI/nvram_common_config.c:          NVRAM_EF_DEMO_APP_CONFIG_SIZE,
custom/common/PLUTO_MMI/nvram_common_config.c:          NVRAM_NORMAL(NVRAM_EF_DEMO_APP_CONFIG_DEFAULT),
custom/common/PLUTO_MMI/nvram_common_config.c:          VER(NVRAM_EF_DEMO_APP_CONFIG_LID)
custom/common/PLUTO_MMI/nvram_common_defs.h:    NVRAM_EF_DEMO_APP_CONFIG_LID,
```

```
S16 error;
demo_app_config config;

ReadRecord(NVRAM_EF_DEMO_APP_CONFIG_LID, 1, (void*)&config, NVRAM_EF_DEMO_APP_CONFIG_SIZE, &error);
config.c2--;
config.c3++;
WriteRecord(NVRAM_EF_DEMO_APP_CONFIG_LID, 1, (void*)&config, NVRAM_EF_DEMO_APP_CONFIG_SIZE, &error);
```
