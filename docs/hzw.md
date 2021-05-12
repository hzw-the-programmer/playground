新建hzw应用目录
MS_Code\MS_MMI\source\mmi_app\app\hzw
在应用目录下建c目录放置c文件
MS_Code\MS_MMI\source\mmi_app\app\hzw\c\hzw.c
在应用目录下建h目录放置头文件
MS_Code\MS_MMI\source\mmi_app\app\hzw\h\hzw.h

为了让源文件参与编译，修改make文件
MS_Code\make\app\app.mk
指定应用c文件目录
MSRCPATH         += MS_MMI/source/mmi_app/app/hzw/c
指定应用头文件目录
MINCPATH         += MS_MMI/source/mmi_app/app/hzw/h
指定c文件
SOURCES        += hzw.c
编译模拟器
make p=project_name m=simulator update

添加module定义
MS_Code\MS_MMI\source\resource\mmi_res_prj_def.h
RES_ADD_MODULE(MMI_MODULE_HZW,"\\hzw\\hzw_mdu_def.h")
指定hzw_mdu_def.h位置
MS_Code\make\resource\resource_header.mk
SOURCES         += hzw_mdu_def.h
SRCPATH  += MS_MMI/source/mmi_app/app/hzw/h
make p=project_name m=resource update
