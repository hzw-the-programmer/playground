# update ui

## file
MS_Code\MS_MMI\source\mmi_app\kernel\c\mmimain.c
## func
APP_Task

## file
MS_Code\MS_MMI\source\mmi_app\app\theme\c\mmitheme_update.c
## func
MMITHEME_UpdateRect
MMITHEME_StoreUpdateRect

# graph

## file
MS_Code\MS_MMI\source\mmi_gui\source\graph\c\guigraph.c
## func
LCD_FillRect
LCD_DrawRect
LCD_FillRoundedRect
LCD_DrawRoundedRect
LCD_FillCircle
LCD_DrawCircle
//LCD_FillPolygon

## file
MS_Code\MS_Ref\export\inc\graphics_draw.h
## func
GRAPH_FillPolygon
GRAPH_FillPolygonEx

## file
MS_Code\MS_MMI\source\mmi_utility\c\mmi_utility.c
## func
MMI_DisplayGraphData

# string

## file
MS_Code\MS_MMI\source\mmi_app\kernel\c\mmi_resource.c
## func
MMIRES_GetText
GetAsciiResText (cache text)

## file
MS_Code\MS_MMI\source\mmi_gui\source\string\c\guistring.c
## func
GUISTR_DrawTextToLCDInRect

# image

## file
MS_Code\MS_MMI\source\mmi_gui\source\res\c\guires_img.c
## func
GUIRES_GetImgWidthHeightByPtr
GUIRES_DisplayImgByPtr

## file
MS_Code\MS_MMI\source\mmi_gui\source\res\c\guires_img.c
## func
GUIRES_GetImgInfo

## file
MS_Code\MS_MMI\source\mmi_utility\c\mmi_utility.c
## func
MMI_GetSabmImgInfo
MMI_GetGraphDataInfo

# scaled image

## file
MS_Code\MS_MMI\source\mmi_gui\source\string\c\guistring.c
## func
GUISTR_DrawSingleLineGradual

## file
MS_Code\MS_MMI\source\mmi_app\app\clipbrd\c\mmiclipbrd_magnify.c
## func
MagnifyContent

## file
MS_Code\MS_MMI\source\mmi_gui\source\iconlist\c\guiiconlist_stunt.c
## func
DrawScaleImage

## file
MS_Code\MS_MMI\source\mmi_app\app\browser\control\src\brw_core_adapter.c
## func
BrwImageDisplayEx

# gradient

## file
MS_Code\MS_MMI\source\mmi_app\app\im\c\mmiim_touch_draw.c
## func
_FillGradientRect
