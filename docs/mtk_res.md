plutommi\mmi\Inc\mmi_res_range_def.h
plutommi\Customer\ResGenerator\ResgenXML\ResgenSrc\mmi_rp_xml.c

plutommi\Customer\Images\FTE240x320\image_{project}.zip

plutommi\mmi\Resource\WriteImgRes.c
Iload
```
if (is_need_convert && flag_file_exist)
{
    S8 tempCommand[SHELL_CMD_BUFLEN];
    //sprintf(tempCommand, ".\\\\convert.exe -channel Alpha -negate -compress none png:%s bmp:%s 2>> .\\debug\\3rd_tool.log", filename_buffer, tempBMPFile);
    sprintf(tempCommand, ".\\\\convert.exe -channel Alpha -negate -compress none png:%s bmp:%s > NUL", filename_buffer, tempBMPFile);
    RES_PRINT_INFO("%s\n", tempCommand);
    ASSERT(SHELL_CMD_BUFLEN > strlen(tempCommand));

    system(tempCommand);
}

Write_Processed_Data
```

MtkWinMainStart

plutommi\Customer\FontResgen\FontResFile.cpp
GenerateFontResFile
GenerateFontResFile_GroupData

plutommi\Customer\CustResource\FontRes.c
g_fontfamily

plutommi\Customer\CustResource\resource_lang_pack_jtbl.c

plutommi\mmi\Resource\StandaloneRes.c
InitializeResourceVariables

vendor\font\inc\L_English_small.h
vendor\font\inc\L_Arabic_small.h
vendor\font\inc\L_gMTKProprietaryFont_small.h
vendor\font\inc\L_English_dialling.h

plutommi\Customer\FontResgen\FontClass.cpp
plutommi\Customer\FontResgen\Fontgen.cpp
AddFont

plutommi\Customer\FontResgen\bdf_operation.cpp
LoadBDFFile

vendor\font\MTK\official\project\plutommi\content\src\MainLcd128X160\res_gen_font.cpp
->
plutommi\Customer\FontResgen\res_gen_font.cpp

plutommi\Customer\ResGenerator_HW.tmp
->
plutommi\Customer\ResGenerator_HW.bat

"..\..\..\Tools\MSYS\bin\make -j16 -fMAKEFILE font_gen.exe -k"
./plutommi/Customer/ResGenerator/Makefile:font_gen.exe: $(FONT_OBJECTS)

del /q ..\FontResgen\font_gen.exe
resgen
