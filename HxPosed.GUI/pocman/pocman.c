#include <stdio.h>
#include "hxposed.h"

#define ZERO_BUF() memset(&reqResp, 0, sizeof(HX_REQUEST_RESPONSE))
#define CHECK_FAIL() if (!CheckFail(&reqResp)) return 1

int CheckFail(PHX_REQUEST_RESPONSE reqResp) {
    if (HxpTrap(reqResp) != 0) {
        printf("Hypervisor is not loaded");
        return -1;
    }

    if (HxIsError(&reqResp->Result)) {
        printf("Failed: %d, %d\n", reqResp->Result.ErrorCode, reqResp->Result.ErrorReason);
        return 1;
    }

    printf("Ok!\n");
    return 0;
}

int main()
{
    printf("HxPosed Tests\n");
    printf("=================[PROCESS TESTS]===================\n");

    printf("Testing HxSvcOpenProcess... ");
    HX_REQUEST_RESPONSE reqResp = {
        .Call.ServiceFunction = HxSvcOpenProcess,
        .OpenObjectRequest = {
            .AddressOrId = 4,
            .OpenType = HxOpenHypervisor
        },
    };

    CHECK_FAIL();

    HX_PROCESS proc = reqResp.OpenObjectResponse.Address;

    printf("Testing HxSvcGetProcessField... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcGetProcessField;
    reqResp.GetProcessFieldRequest.Address = proc;
    reqResp.GetProcessFieldRequest.Data.Field = HxProcFieldThreads;

    CHECK_FAIL();

    UINT32 len = 0;
    PUINT32 threads_slice = HxReadAsyncResponseSlice(reqResp.GetProcessFieldResponse.ThreadsOffset, &len);
    PUINT32 threads = HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, len * 4);
    memcpy(threads, threads_slice, len * 4);


    printf("Testing HxSvcSetProcessField... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcSetProcessField;
    reqResp.SetProcessFieldRequest.Address = proc;
    reqResp.SetProcessFieldRequest.Data.Field = HxProcFieldProtection;
    reqResp.SetProcessFieldRequest.Data.Protection = (HX_PROCESS_PROTECTION){
        .Audit = FALSE,
        .Level = HxPsSigUnchecked,
        .Type = HxPsProtTypeNone,
    };

    CHECK_FAIL();


    printf("Testing HxSvcCloseProcess... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcCloseProcess;
    reqResp.CloseObjectRequest.Address = proc;

    CHECK_FAIL();

    printf("=================[THREAD TESTS]===================\n");
    printf("Testing HxSvcOpenThread... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcOpenThread;
    reqResp.OpenObjectRequest.AddressOrId = threads[0];
    reqResp.OpenObjectRequest.OpenType = HxOpenHypervisor;

    CHECK_FAIL();

    HX_THREAD thread = reqResp.OpenObjectResponse.Address;

    printf("Testing HxSvcGetThreadField... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcGetThreadField;
    reqResp.GetThreadFieldRequest.Address = thread;
    reqResp.GetThreadFieldRequest.Data.Field = HxThreadFieldActiveImpersonationInfo;

    CHECK_FAIL();

    printf("Testing HxSvcCloseThread... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcCloseThread;
    reqResp.CloseObjectRequest.Address = thread;

    CHECK_FAIL();

    printf("Skipping dangerous tests...\n");


    printf("=================[TOKEN TESTS]===================\n");
    printf("Testing HxSvcOpenToken... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcOpenToken;
    reqResp.OpenObjectRequest.AddressOrId = 0; // tells to open system token
    reqResp.OpenObjectRequest.OpenType = HxOpenHypervisor;

    CHECK_FAIL();

    HX_TOKEN token = reqResp.OpenObjectResponse.Address;

    printf("Testing HxSvcSetTokenField... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcSetTokenField;
    reqResp.SetTokenFieldRequest.Address = token;
    reqResp.SetTokenFieldRequest.Data.Field = HxTokenFieldPresentPrivileges;
    reqResp.SetTokenFieldRequest.Data.Privileges = (HX_TOKEN_PRIVILEGES){
        .All = MAXUINT64
    };

    CHECK_FAIL();

    printf("Testing HxSvcGetTokenField... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcGetTokenField;
    reqResp.GetTokenFieldRequest.Address = token;
    reqResp.GetTokenFieldRequest.Data.Field = HxTokenFieldPresentPrivileges;

    CHECK_FAIL();

    if (reqResp.GetTokenFieldResponse.Privileges.All != MAXUINT64) {
        printf("Inconsisten results");
        return 1;
    }

    printf("Testing HxSvcCloseToken");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcCloseToken;
    reqResp.CloseObjectRequest.Address = token;

    CHECK_FAIL();

    printf("=================[CPU/IO TESTS]===================\n");
}