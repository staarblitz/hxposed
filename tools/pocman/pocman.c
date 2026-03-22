#include "hxposed.h"
#include "Psapi.h"

#define CHECK_FAIL() 

int main()
{
    HX_PROCESS sys;
    HX_RESULT result = HxOpenObject(HxSvcOpenProcess, 680, &sys);
    if (result.ErrorCode != 0) {

        return -1;
    }

    HANDLE hProcess = OpenProcess(PROCESS_VM_READ, FALSE, GetCurrentProcessId());

    CHAR pathBuf[256];
    if (GetProcessImageFileNameA(hProcess, pathBuf, 256) != 0) {
        return -2;
    }

    result = HxUpgradeHandle(hProcess, 0, 0x1FFFFFF);
    if (result.ErrorCode != 0) {

        return -1;
    }

    if (GetProcessImageFileNameA(hProcess, pathBuf, 256) == 0) {
        return -2;
    }

    result = HxSwapHandleObject(hProcess, 0, sys);
    if (result.ErrorCode != 0) {

        return -1;
    }

    if (GetProcessImageFileNameA(hProcess, pathBuf, 256) == 0) {
        return -2;
    }
    return 0;
}