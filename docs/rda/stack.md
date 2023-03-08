static void task_test() {
	UINT8 id = sxr_GetCurrentTaskId();
	UINT8 *top = sxr_GetStatckTop(id);
	UINT8 *bottom = sxr_GetStatckBottom(id);

	mmi_trace(MMI_TRACE_LEVEL_1, "hzw:%s,id=%d,top=%p,bottom=%p,size=%d,usage=%d",
		__func__, id, top, bottom, bottom - top, sxr_GetStackMaxUsage(id));
}

sxr_Task.Ctx [Id].StackTop = (u32 *)_sxr_HMalloc ((u16)(SXR_SET_STACK(Desc -> StackSize) << 2), SXR_TK_STCK_HEAP);

u32 sxr_GetStackMaxUsage(u8 id)
{
    u32 i=0;
    u32 *stackMem = (u32 *)sxr_Task.Ctx [id].StackTop;
    while (stackMem[i++] == SXR_DW_MEM_PATTERN );

    return (SXR_SET_STACK(sxr_Task.Ctx [id].Desc -> StackSize) << 2) - (i << 2);
}