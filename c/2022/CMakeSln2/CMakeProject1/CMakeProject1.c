#include <windows.h>
#include "CMakeProject1.h"

typedef void (WINAPI* PGNSI)(LPSYSTEM_INFO);

int main()
{

    // Call GetNativeSystemInfo if supported or GetSystemInfo otherwise.

    PGNSI pGNSI;
    SYSTEM_INFO si;

    ZeroMemory(&si, sizeof(SYSTEM_INFO));

    pGNSI = (PGNSI)GetProcAddress(
        GetModuleHandle(TEXT("kernel32.dll")),
        "GetNativeSystemInfo");
    if (NULL != pGNSI)
    {
        pGNSI(&si);
    }
    else
    {
        GetSystemInfo(&si);
    }
    return 0;
}
