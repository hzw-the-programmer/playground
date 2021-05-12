interface/inet_ps/soc_api.h
interface/hal/cipher/include/che_api.h

plutommi\Framework\GUI\GUI_SRC\wingui.c
setup_UI_wrappers
plutommi\Framework\EventHandling\EventsSrc\MMITimer.c

plutommi\Framework\GUI\GUI_SRC\gui_windows.c
gui_create_scrolling_text

plutommi\Framework\Fontengine\PixcomFontEngine.c

```
config\include\app\app_task_config.h
/*************************Task CFG Begin****************/
/*task_indx_type*/
task_index(INDX_WAP) 
/*module_type and mod_task_g*/
task_module_map(INDX_WAP, MOD_WAP)
/* MOD_WAP is used by HAL file, please don't delete or modify it */
/*task's parameters*/
task_name("WAP")
task_queue_name("WAP Q")
task_priority(TASK_PRIORITY_WAP)
task_stack_size(3894)
#ifndef WAP_NOT_PRESENT
task_create_function(wap_create)
#else
null_task_create_entry(NULL)
#endif
task_stack_internalRAM(KAL_FALSE)
task_external_queue_size(20)
task_internal_queue_size(0)
task_boot_mode(NORMAL_M)
/*************************Task CFG END******************/

vendor\wap\obigo_Q03C\adaptation\integration\source\wap_create.c
wap_create
wap_task_main
wap_init_framework

vendor\wap\obigo_Q03C\adaptation\msf_ui\src\widget.c
widget_init
```

```
config\include\app\app_task_config.h
/*************************Task CFG Begin****************/
/*task_indx_type*/
task_index(INDX_MMI) 
/*module_type and mod_task_g*/
#ifdef WISDOM_MMI
/* under construction !*/
/* under construction !*/
#else
task_module_map(INDX_MMI, MOD_MMI)
#endif
/* MOD_MMI is used by HAL file, please don't delete or modify it */

/*task's parameters*/
task_name("MMI")
task_queue_name("MMI Q")
task_priority(TASK_PRIORITY_MMI)

#if defined(NEPTUNE_MMI)
#if defined(__LOW_COST_SUPPORT_ULC__)
task_stack_size(3072)
#else /* __LOW_COST_SUPPORT_ULC__ */
task_stack_size(4096)
#endif /* __LOW_COST_SUPPORT_ULC__ */
#else  /* Default value */
#if defined(WISDOM_MMI)
#if defined(OPERA_BROWSER) /* MMI: + 4 KB */
/* under construction !*/
#else
/* under construction !*/
#endif
#else  /* WISDOM_MMI */
#ifdef __MOBILE_VIDEO_SUPPORT__
task_stack_size(20480)
#else
#if defined(__MRE_PACKAGE_FULL__) || defined(__MRE_PACKAGE_NORMAL__)
task_stack_size(16348) /* MRE APP opera need 16KB */
#elif defined(OPERA_BROWSER) /* MMI: + 4 KB */
task_stack_size(10240)
#else
task_stack_size(6144)
#endif
#endif
#endif /* WISDOM_MMI */
#endif /* NEPTUNE_MMI */    
#ifdef MMI_NOT_PRESENT
null_task_create_entry(NULL)
#elif !defined(WISDOM_MMI)
task_create_function(mmi_create)
#else
/* under construction !*/
#endif
task_stack_internalRAM(KAL_FALSE)
#if defined(WISDOM_MMI)
/* under construction !*/
/* under construction !*/
#else
   #if (defined(__GEMINI__)) && (GEMINI_PLUS >= 4)
      task_external_queue_size(40)
   #else
      task_external_queue_size(30)
   #endif
task_internal_queue_size(0)
#endif
task_boot_mode(NORMAL_M | USB_M)
/*************************Task CFG END******************/

plutommi\Framework\Tasks\TasksSrc\TaskInit.c
mmi_create

plutommi\Framework\Tasks\TasksSrc\MMITask.c
MMI_Init

plutommi\Framework\EventHandling\EventsSrc\MMITimer.c
L4InitTimer
plutommi\Framework\GUI\GUI_SRC\wingui.c
setup_UI_wrappers
plutommi\Framework\Fontengine\PixcomFontEngine.c
mmi_fe_init
```

```
kal\include\kal_public_api.h
kal_create_mutex
kal_take_mutex
kal_give_mutex

kal_create_sem
kal_take_sem
kal_give_sem
```
