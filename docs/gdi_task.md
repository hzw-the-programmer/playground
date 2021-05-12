#files:#

plutommi/Framework/GDI/GDISrc/gdi_task.c
config/include/hal/stack_config.h
task_indx_type
module_type

config/src/hal/syscomp_config.c
mod_task_g
sys_comp_config_tbl

config/include/app/app_task_config.h

#call stack 1:#

_threadstartex
_callthreadstartex
MtkWinMainStart
_mainp
_stack_init

##create tasks##
stack_init_comp_info
gdi_create_task

##init tasks##
_stack_init_tasks
gdi_task_init

#call stack 2:#

_threadstartex
_callthreadstartex
_osc_platform_thread_create
gdi_task_main
