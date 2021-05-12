project file:
project/MS_Code/project_project_name.mk

build resource:
#make p=project_name m=resource clean
make p=project_name m=resource update

build simulator:
#make p=project_name m=simulator clean
make p=project_name m=simulator update

vc project file:
project\MS_Code\build\project_name_builddir\win\simulator.dsw
set msdevkernel as active project


