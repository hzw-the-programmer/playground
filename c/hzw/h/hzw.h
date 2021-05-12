#ifndef __HZW_H__
#define __HZW_H__

#include "mmi_module.h"

typedef enum
{
	MMIHZW_WIN_ID_START = (MMI_MODULE_HZW << 16),
	MMIHZW_WIN_ID_HZWMAIN,
} MMIHZW_WINDOW_ID_E;

extern void EntryHZW(void);

#endif
