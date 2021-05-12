#include "window_parse.h"
#include "mmk_type.h"
#include "mmk_app.h"
#include "guiblock.h"
#include "mmidisplay_data.h"

#include "hzw.h"

/*---------------------------------------------------------------------------*/
/*                          LOCAL FUNCTION DECLARE                           */
/*---------------------------------------------------------------------------*/

void EntryHZW(void);
MMI_RESULT_E HandleHZWMainWinMsg(MMIHZW_WINDOW_ID_E win_id, MMI_MESSAGE_ID_E msg_id, DPARAM param);

WINDOW_TABLE(MMIHZW_MAIN_WIN_TAB) =
{
	WIN_ID(MMIHZW_WIN_ID_HZWMAIN),
	WIN_FUNC((uint32)HandleHZWMainWinMsg),
	END_WIN,
};

/**--------------------------------------------------------------------------*
 **                         FUNCTION DEFINITION                              *
 **--------------------------------------------------------------------------*/

void EntryHZW(void)
{
	MMK_CreateWin((uint32*)MMIHZW_MAIN_WIN_TAB,	PNULL);
}

MMI_RESULT_E HandleHZWMainWinMsg(MMIHZW_WINDOW_ID_E win_id, MMI_MESSAGE_ID_E msg_id, DPARAM param)
{
	MMI_RESULT_E result = MMI_RESULT_TRUE;

	switch (msg_id)
    {
    	case MSG_FULL_PAINT:
    	{
    		GUI_RECT_T rect = {0, 0, 239, 319};
			GUI_LCD_DEV_INFO dev_info = {GUI_MAIN_LCD_ID, GUI_BLOCK_MAIN};
			GUI_FillRect(&dev_info, rect, MMI_RED_COLOR);
			break;
		}
	}

	return result;
}

