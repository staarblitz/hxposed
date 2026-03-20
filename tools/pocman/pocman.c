#include <stdio.h>
#include <intrin.h>
#include "hxposed.h"

#define ZERO_BUF() memset(&reqResp, 0, sizeof(HX_REQUEST_RESPONSE))
#define CHECK_FAIL() if (CheckFail(&reqResp) != 0) return 1

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
            .AddressOrId = GetCurrentProcessId(),
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
    printf("Testing HxSvcMsrIo... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcMsrIo;
    reqResp.MsrIoRequest.Msr = 0x10;
    reqResp.MsrIoRequest.Operation = HxMsrRead;

    CHECK_FAIL();

    printf("Testing HxSvcExecutePrivilegedInstruction... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcExecutePrivilegedInstruction;
    reqResp.ExecutePrivilegedInstructionRequest.Instruction = HxPiCli;

    CHECK_FAIL();

    UINT64 eflags = __readeflags();
    if (eflags & (1 << 9) != 1) {
        printf("Interrupt flag isn't set!");
        return 1;
    }

    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcExecutePrivilegedInstruction;
    reqResp.ExecutePrivilegedInstructionRequest.Instruction = HxPiSti;

    CHECK_FAIL();

    eflags = __readeflags();
    if (eflags & (1 << 9) != 0) {
        printf("Interrupt flag is still set!");
        return 1;
    }

    printf("=================[MEMORY TESTS]===================\n");
    printf("Testing HxSvcAllocateMemory... ");

    reqResp.Call.ServiceFunction = HxSvcAllocateMemory;
    reqResp.AllocateMemoryRequest.Pool = HxPoolNonPaged;
    reqResp.AllocateMemoryRequest.Size = 4096;

    CHECK_FAIL();

    HX_RMD alloc = reqResp.AllocateMemoryResponse.RawMemoryDescriptor;

    printf("Testing HxSvcMapRawMemoryDescriptor... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcMapRawMemoryDescriptor;
    reqResp.MapRawMemoryDescriptorRequest.AddressSpace = proc;
    reqResp.MapRawMemoryDescriptorRequest.MapAddress = 0x13370000;
    reqResp.MapRawMemoryDescriptorRequest.Operation = HxMemMap;
    reqResp.MapRawMemoryDescriptorRequest.MemoryDescriptor = alloc;

    CHECK_FAIL();

    PUINT64 hey = 0x13370000;
    *hey = 0x2009;

    printf("Testing HxSvcTranslateAddress... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcTranslateAddress;
    reqResp.TranslateAddressRequest.AddressSpace = proc;
    reqResp.TranslateAddressRequest.VirtualAddress = 0x13370000;

    CHECK_FAIL();

    printf("Testing HxSvcFreeMemory... ");
    ZERO_BUF();

    reqResp.Call.ServiceFunction = HxSvcFreeMemory;
    reqResp.FreeMemoryRequest.Object = alloc;

    CHECK_FAIL();

    printf("=================[TESTS COMPLETED]===================\n");
    printf("This tool includes only tests thatt are very less integrated with each other. Please use the GUI tool better tests.");

    return 0;
}