#include "hxposed.h"

#define CHECK_FAIL() 

int main()
{
    HX_PROCESS sys;
    HX_RESULT result = HxOpenObject(HxSvcOpenProcess, 4, &sys);
    if (result.ErrorCode != 0) {

        return -1;
    }

    HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, GetCurrentProcessId());
    result = HxUpgradeHandle(hProcess, 0, 0x1FFFFFF);
    if (result.ErrorCode != 0) {

        return -1;
    }

    result = HxSwapHandleObject(hProcess, 0, sys);
    if (result.ErrorCode != 0) {

        return -1;
    }
    return 0;
}