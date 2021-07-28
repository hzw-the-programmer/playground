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

```
#define PATH_LEN 50

static void getFree(U8 drv, U64 *free) {
#if 1
    *free += 10*1024;
#else
    srv_fmgr_drv_get_size(drv, NULL, free);
#endif
}

static U64 reserve(U64 target, const WCHAR *dir, const WCHAR *ext) {
    U64 free = 0;
    WCHAR path[PATH_LEN + 1];
    FS_HANDLE handle;
    FS_DOSDirEntry entry;
    WCHAR fn[PATH_LEN + 1];

    getFree(dir[0], &free);
    if (free >= target) {
        return free;
    }

    kal_wstrcpy(path, dir);
    kal_wstrcat(path, L"*.");
    kal_wstrcat(path, ext);
    
    handle = FS_FindFirst(
                path, 
                0, 
                0, 
                &entry, 
                fn, 
                sizeof(fn));

    if (handle <= 0) {
        return free;
    }

    do {
        kal_wstrcpy(path, dir);
        kal_wstrcat(path, fn);

        FS_Delete(path);

        getFree(dir[0], &free);
    } while (free < target && FS_FindNext(handle, &entry, fn, sizeof(fn)) == FS_NO_ERROR);

    FS_FindClose(handle);

    return free;
}

void reserve_test() {
    reserve(32 * 1024, L"E:\\bj", L"tj");
}
```

```
#define PATH_LEN 50

static void getFree(U8 drv, U64 *free) {
#if 0
    *free += 10*1024;
#else
    srv_fmgr_drv_get_size(drv, NULL, free);
#endif
}

static U64 reserve(U64 target, const WCHAR *dir, const WCHAR *ext) {
    U64 free = 0;
    WCHAR path[PATH_LEN + 1];
    FS_HANDLE handle;
    FS_DOSDirEntry entry;
    WCHAR fn[PATH_LEN + 1];

    getFree(dir[0], &free);
    if (free >= target) {
        return free;
    }

    kal_wstrcpy(path, dir);
    kal_wstrcat(path, L"*.");
    kal_wstrcat(path, ext);
    
    handle = FS_FindFirst(
                path, 
                0, 
                0, 
                &entry, 
                fn, 
                sizeof(fn));

    if (handle <= 0) {
        return free;
    }

    do {
        kal_wstrcpy(path, dir);
        kal_wstrcat(path, fn);

        kal_prompt_trace(MOD_ABM, "delete: %d", (S32)entry.FileSize);
        FS_Delete(path);

        getFree(dir[0], &free);
    } while (free < target && FS_FindNext(handle, &entry, fn, sizeof(fn)) == FS_NO_ERROR);

    FS_FindClose(handle);

    return free;
}

void reserve_test() {
    U64 total, free;
    U32 start;

    srv_fmgr_drv_get_size(SRV_FMGR_PHONE_DRV, &total, &free);
    kal_prompt_trace(MOD_ABM, "%d, %d, %d", SRV_FMGR_PHONE_DRV, (S32)total, (S32)free);
    srv_fmgr_drv_get_size(SRV_FMGR_CARD_DRV, &total, &free);
    kal_prompt_trace(MOD_ABM, "%d, %d, %d", SRV_FMGR_CARD_DRV, (S32)total, (S32)free);

    start = kal_get_systicks();
    free = reserve(1773800000, L"E:\\tj", L"bj");    
    kal_prompt_trace(MOD_ABM, "cost=%d, free=%d", kal_get_systicks() - start, (S32)free);
}
```

```
typedef struct {
    WCHAR fn[PATH_LEN + 1];
    U32 ts;
} FileInfo;

static void convertDosDt(FS_DOSDateTime *dosdt, applib_time_struct *dt) {
    dt->nYear = dosdt->Year1980 + 1980;
    dt->nMonth = dosdt->Month;
    dt->nDay = dosdt->Day;
    dt->nHour = dosdt->Hour;
    dt->nMin = dosdt->Minute;
    dt->nSec = dosdt->Second2;
    dt->DayIndex = 0;
}

static void get20Oldest(const WCHAR *pattern) {
#define cap 3
    FS_HANDLE handle;
    FS_DOSDirEntry entry;
    WCHAR fn[PATH_LEN + 1];
    FileInfo fis[cap + 1];
    U32 len = 0;
    U32 i, ts;
    applib_time_struct dt;

    memset(&fis[cap], 0, sizeof(fis[0]));

    handle = FS_FindFirst(
                pattern, 
                0, 
                0, 
                &entry, 
                fn, 
                sizeof(fn));

    if (handle <= 0) {
        return;
    }

    do {
        convertDosDt(&entry.CreateDateTime, &dt);
        ts = applib_dt_mytime_2_utc_sec(&dt, 0);
        for (i = 0; i < len; i++) {
            if (ts < fis[i].ts) {
                break;
            }
        }
        if (i == cap) {
            continue;
        }
        memmove(&fis[i + 1], &fis[i], (cap - i - 1) * sizeof(fis[0]));
        kal_wstrcpy(fis[i].fn, fn);
        fis[i].ts = ts;
        if (len < cap) {
            len++;
        }
    } while (FS_FindNext(handle, &entry, fn, sizeof(fn)) == FS_NO_ERROR);

    FS_FindClose(handle);

#undef cap
}
```
