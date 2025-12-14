#include <stdio.h>
#include "hxposed.h"

int main()
{
    UINT128 guid = {
        .Low =  14562116945099443309,
        .High = 11550595627064306744
    };

    HXR_AUTH auth = {
        .Guid = guid,
        .Permissions = MAXUINT64
    };

    HXS_AUTH response;
    DebugBreak();

    PHX_REQUEST_RESPONSE raw = HxpRawFromRequest(HxSvcAuthorize, &auth);

    HxpTrap(raw, NULL);

    HX_ERROR error = HxpResponseFromRaw(raw, &response);
    if (HxIsError(&error)) {
        printf("lol");
    }

    HXR_OPEN_PROCESS open = {
        .Id = 6892,
        .OpenType = HxOpenHandle,
    };

    HX_ASYNC_INFO async = {
        .Handle = HxpCreateEventHandle()
    };

    raw = HxpRawFromRequest(HxSvcOpenProcess, &open);

    HxpTrap(raw, &async);

    WaitForSingleObject(async.Handle, INFINITE);

    HXS_OPEN_OBJECT_RESPONSE process;
    error = HxpResponseFromAsync(&async, &process);

    if (HxIsError(&error)) {
        printf("fail");
    }

    TerminateProcess(process.Address, 0);
}