"C:\Program Files\DS-5 v5.22.0\sw\ARMCompiler5.05u2/bin/armlink"
--cpu cortex-a7
--entry __vectors
--map
--symbols
--scatter build/project_240X320BAR_48MB_CAT1_builddir/tmp/link_PROJECT_project_barphone_VM.scf
--no_merge
--feedback build/project_240X320BAR_48MB_CAT1_builddir/project_240X320BAR_48MB_CAT1_feedback.txt
--xref
--remove
--pad 0x00
--verbose
--list PROJECT_project_240X320BAR_48MB_CAT1.map
--info sizes
--via build/project_240X320BAR_48MB_CAT1_builddir/tmp/link_lib.txt
-o build/project_240X320BAR_48MB_CAT1_builddir/img/PROJECT_project_240X320BAR_48MB_CAT1.axf
2>&1 | "make\make_cmd\tee" build/project_240X320BAR_48MB_CAT1_builddir/log/link.log
&&
"C:\Program Files\DS-5 v5.22.0\sw\ARMCompiler5.05u2/bin/fromelf"
-c
--bin build/project_240X320BAR_48MB_CAT1_builddir/img/PROJECT_project_240X320BAR_48MB_CAT1.axf
--output build/project_240X320BAR_48MB_CAT1_builddir/img/PROJECT_project_240X320BAR_48MB_CAT1
2>&1 | "make\make_cmd\tee" build/project_240X320BAR_48MB_CAT1_builddir/log/fromelf.log