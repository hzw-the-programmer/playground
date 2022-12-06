$ grep "soc_polling_timer_expiry" -r .
Binary file ./MoDIS_VC9/drv_sim/Debug/drv_sim.lib matches
Binary file ./MoDIS_VC9/drv_sim/Debug/w32_socket.obj matches
Binary file ./MoDIS_VC9/drv_sim/Release/drv_sim.lib matches
Binary file ./MoDIS_VC9/drv_sim/Release/w32_socket.obj matches
./MoDIS_VC9/drv_sim/src/w32_socket.c:void soc_polling_timer_expiry(void)
Binary file ./MoDIS_VC9/MoDIS/Debug/MoDIS.ilk matches
./MoDIS_VC9/MoDIS/Debug/MoDIS.map: 0002:00650870       _soc_polling_timer_expiry  0133e870 f   drv_sim:w32_socket.obj
Binary file ./MoDIS_VC9/MoDIS/Debug/MoDIS.pdb matches
grep: ./MoDIS_VC9/MoDIS.ncb: Device or resource busy
Binary file ./MoDIS_VC9/MoDIS_LIB/MT6260/S00/gprs/GEMINI/2/soc.lib matches
Binary file ./MoDIS_VC9/MoDIS_LIB/MT6260/S00/gprs/GEMINI/FALSE/soc.lib matches

$ grep "soc_expiry_hanlder" -r .
./MoDIS_VC9/drv_sim/src/w32_socket.c: * Called by soc_expiry_hanlder() at the condition of
grep: ./MoDIS_VC9/MoDIS.ncb: Device or resource busy

$ grep "soc_init_win32" -r .
Binary file ./MoDIS_VC9/drv_sim/Debug/drv_sim.lib matches
Binary file ./MoDIS_VC9/drv_sim/Debug/w32_socket.obj matches
Binary file ./MoDIS_VC9/drv_sim/Release/drv_sim.lib matches
Binary file ./MoDIS_VC9/drv_sim/Release/w32_socket.obj matches
./MoDIS_VC9/drv_sim/src/w32_socket.c:void soc_init_win32(void)
Binary file ./MoDIS_VC9/MoDIS/Debug/MoDIS.ilk matches
./MoDIS_VC9/MoDIS/Debug/MoDIS.map: 0002:006508e0       _soc_init_win32            0133e8e0 f   drv_sim:w32_socket.obj
Binary file ./MoDIS_VC9/MoDIS/Debug/MoDIS.pdb matches
grep: ./MoDIS_VC9/MoDIS.ncb: Device or resource busy
Binary file ./MoDIS_VC9/MoDIS_LIB/MT6260/S00/gprs/GEMINI/2/soc.lib matches
Binary file ./MoDIS_VC9/MoDIS_LIB/MT6260/S00/gprs/GEMINI/FALSE/soc.lib matches

$ grep "soc_timeout_win32" -r .
Binary file ./MoDIS_VC9/drv_sim/Debug/drv_sim.lib matches
Binary file ./MoDIS_VC9/drv_sim/Debug/w32_socket.obj matches
Binary file ./MoDIS_VC9/drv_sim/Release/drv_sim.lib matches
Binary file ./MoDIS_VC9/drv_sim/Release/w32_socket.obj matches
./MoDIS_VC9/drv_sim/src/w32_socket.c:static struct timeval soc_timeout_win32;
./MoDIS_VC9/drv_sim/src/w32_socket.c:        rt = select(max_fd, &read_fd, &write_fd, &except_fd, &soc_timeout_win32);
./MoDIS_VC9/drv_sim/src/w32_socket.c:        soc_timeout_win32.tv_sec = 0;
./MoDIS_VC9/drv_sim/src/w32_socket.c:        soc_timeout_win32.tv_usec = 50; /* wait for 50 usec */
Binary file ./MoDIS_VC9/MoDIS/Debug/MoDIS.pdb matches
grep: ./MoDIS_VC9/MoDIS.ncb: Device or resource busy

$ grep "soc_start_timer" -r .
Binary file ./MoDIS_VC9/drv_sim/Debug/drv_sim.lib matches
Binary file ./MoDIS_VC9/drv_sim/Debug/w32_socket.obj matches
Binary file ./MoDIS_VC9/drv_sim/Release/drv_sim.lib matches
Binary file ./MoDIS_VC9/drv_sim/Release/w32_socket.obj matches
./MoDIS_VC9/drv_sim/src/w32_socket.c:void soc_start_timer(kal_uint8 timer_id,
./MoDIS_VC9/drv_sim/src/w32_socket.c:        soc_start_timer(0 /* dummy timer_id */,
Binary file ./MoDIS_VC9/MoDIS/Debug/MoDIS.ilk matches
./MoDIS_VC9/MoDIS/Debug/MoDIS.map: 0002:00a0be10       _soc_start_timer           016f9e10 f   soc:soc_timer.obj
Binary file ./MoDIS_VC9/MoDIS/Debug/MoDIS.pdb matches
grep: ./MoDIS_VC9/MoDIS.ncb: Device or resource busy
Binary file ./MoDIS_VC9/MoDIS_LIB/MT6260/S00/gprs/GEMINI/2/soc.lib matches
Binary file ./MoDIS_VC9/MoDIS_LIB/MT6260/S00/gprs/GEMINI/FALSE/soc.lib matches
Binary file ./mtk_lib/MT6260/S00/gprs/GEMINI/2/soc.lib matches
Binary file ./mtk_lib/MT6260/S00/gprs/GEMINI/FALSE/soc.lib matches
