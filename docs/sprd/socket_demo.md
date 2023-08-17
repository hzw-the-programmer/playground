MS_MMI_Main\source\mmi_app\kernel\h\mmk_regapp.def
```
#if 1
REG_APP(SOCKET_SIG_BEGIN, SOCKET_SIG_END, &g_demo_app)
#endif
```

MS_MMI_Main\source\mmi_app\kernel\h\mmk_ext_app.h
```
#if 1
    extern MMI_APPLICATION_T g_demo_app;
#endif
```

MS_MMI_Main\source\mmi_app\kernel\c\mmimain.c
```
#if 1
    demo_app_init();
#endif
```

MS_MMI_Main\source\resource\mmi_res_prj_def.h
```
#if 1
RES_ADD_MODULE(MMI_MODULE_DEMOAPP,"\\demoapp\\demoapp_mdu_def.h")
#endif
```