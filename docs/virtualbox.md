lsmod | grep guest
modinfo vboxguest

1. 扩展虚拟硬盘。打开Virtual Box（虚拟机处于关闭状态）
管理->虚拟介质管理->虚拟硬盘->属性->大小
2. 扩展分区。打开虚拟机
开始->控制面板->管理工具->计算机管理->存储->磁盘管理
右键想要扩充的盘，选择扩展卷

shared folder:
ls -l /media
drwxrwx---  1 root vboxsf    0 11月 11 15:42 sf_proto

groups
sudo adduser hezhiwen vboxsf
groups
reboot

cd /c/Program Files/Oracle/VirtualBox
./VBoxManage.exe list vms
./VBoxManage.exe modifymedium eabc91c5-dc1a-4402-a899-aa26fbe48b23 --resize <megabytes>
gparted
