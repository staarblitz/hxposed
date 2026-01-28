#include <stdio.h>
#include "hxposed.h"

int main()
{
    HXR_OPEN_PROCESS open = {
        .Id = 6892,
        .OpenType = HxOpenHandle,
    };

    PHX_REQUEST_RESPONSE raw = HxpRawFromRequest(HxSvcOpenProcess, &open);

    if (HxpTrap(raw) == 0) {
        printf("hv not loaded");
        return 1;;
    }

    HXS_OPEN_OBJECT_RESPONSE process;
    HX_ERROR error = HxpResponseFromRaw(raw, &process);
    if (HxIsError(&error)) {
        printf("fail");
    }

    TerminateProcess(process.Address, 0);
}