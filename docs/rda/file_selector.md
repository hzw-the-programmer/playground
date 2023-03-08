FMGR_FILTER filter;
FMGR_FILTER_INIT(&filter);
FMGR_FILTER_SET(&filter, FMGR_TYPE_JPG);
FMGR_FILTER_SET(&filter, FMGR_TYPE_JPEG);
FMGR_FILTER_SET(&filter, FMGR_TYPE_PNG);
FMGR_FILTER_SET(&filter, FMGR_TYPE_FOLDER);
mmi_fmgr_select_path_and_enter(
    APP,
    FMGR_SELECT_FILE | FMGR_SELECT_REPEAT,
    filter,
    (S8*) L"root",
    cb);