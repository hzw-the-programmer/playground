```
#define PATH_LEN 50

static U32 getTotalSize(WCHAR *pat) {
    U32 size = 0;
    FS_HANDLE handle;
    FS_DOSDirEntry entry;
    S32 ret;
    WCHAR fn[PATH_LEN + 1];

    handle = FS_FindFirst(
                pat, 
                0, 
                0, 
                &entry, 
                fn, 
                sizeof(fn));

    if (handle <= 0) {
        return;
    }

    do {
        size += entry.FileSize;

        ret = FS_FindNext(
                handle, 
                &entry,
                fn,
                sizeof(fn));
    } while (ret == FS_NO_ERROR);

    FS_FindClose(handle);

    return size;
}

static U32 getFreeSpace(const WCHAR *drive) {
    S32 ret;
    FS_DiskInfo info;

    ret = FS_GetDiskInfo(drive, &info, FS_DI_BASIC_INFO | FS_DI_FREE_SPACE);
    if (ret >= FS_NO_ERROR)
    {
        return info.FreeClusters * info.SectorsPerCluster * info.BytesPerSector;
    }

    return 0;
}

void fs_test1() {
    U64 total, free;
    S32 ret;

    ret = srv_fmgr_drv_get_size(SRV_FMGR_PHONE_DRV, &total, &free);
    kal_prompt_trace(MOD_ABM, "ret=%d, total=%d, free=%d", ret, total, free);

    ret = srv_fmgr_drv_get_size(SRV_FMGR_CARD_DRV, &total, &free);
    kal_prompt_trace(MOD_ABM, "ret=%d, total=%d, free=%d", ret, total, free);
}

void fs_test() {
    fs_test1();
}
```
