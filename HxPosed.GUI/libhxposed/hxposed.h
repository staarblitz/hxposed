#pragma once

#ifndef C_IS_GARBAGE
#define C_IS_GARBAGE


#include <stdint.h>
#include <Windows.h>

typedef struct _UINT128 {
    UINT64 Low;
    UINT64 High;
} UINT128, *PUINT128;

typedef enum _HX_ERROR_SOURCE {
    HxSourceNt = 0,
    HxSourceHv = 1,
    HxSourceHx = 2
} HX_ERROR_SOURCE;

typedef enum _HX_ERROR_CODE {
    HxErrUnknown = 0,
    HxErrOk = 1,
    HxErrNotAllowed = 2,
    HxErrNotLoaded = 3,
    HxErrNotFound = 4,
    HxErrInvalidParams = 5
} HX_ERROR_CODE;

typedef enum _HX_SERVICE_FUNCTION {
    HxSvcUnknown = 0,
    HxSvcAuthorize = 1,
    HxSvcGetState = 2,
    HxSvcOpenProcess = 3,
    HxSvcCloseProcess = 4,
    HxSvcKillProcess = 5,
    HxSvcAddAsyncHandler = 6,
    HxSvcRemoveAsyncHandler = 7,
    HxSvcGetProcessField = 8,
    HxSvcSetProcessField = 9,
    HxSvcProcessVMOperation = 10,
    HxSvcProtectProcessMemory = 11,
    HxSvcAllocateMemory = 12,
    HxSvcMapMemory = 13,
    HxSvcFreeMemory = 14,
    HxSvcGetProcessThreads = 15,
    HxSvcOpenThread = 16,
    HxSvcCloseThread = 17,
    HxSvcSuspendResumeThread = 18,
    HxSvcKillThread = 19,
    HxSvcGetSetThreadContext = 20,
    HxSvcGetThreadField = 21,
    HxSvcSetThreadField = 22,
    HxSvcOpenToken = 23,
    HxSvcGetTokenField = 24,
    HxSvcCloseToken = 25,
    HxSvcSetTokenField = 26,
} HX_SERVICE_FUNCTION;

typedef struct _HX_ERROR {
    HX_ERROR_SOURCE ErrorSource;
    UINT16 ErrorCode;
    UINT16 ErrorReason;
} HX_ERROR, *PHX_ERROR;

#pragma pack(push,1)
typedef struct _HX_RESULT {
    HX_SERVICE_FUNCTION ServiceFunction : 16;
    HX_ERROR_SOURCE ErrorSource : 2;
    HX_ERROR_CODE ErrorCode : 3;
    UINT32 Reserved : 11;
} HX_RESULT, *PHX_RESULT;

typedef struct _HX_CALL {
    HX_SERVICE_FUNCTION ServiceFunction : 16;
    BOOL IsFast : 1;
    BOOL IgnoreResult : 1;
    BOOL YieldExecution : 1;
    BOOL IsAsync : 1;
    BOOL ExtendedArgsPresent : 1;
    UINT32 Reserved : 10;
} HX_CALL, *PHX_CALL;

typedef struct _HX_REQUEST_RESPONSE {
    HX_CALL Call;
    HX_RESULT Result;
    // padding for the first member of shared memory region
    UINT32 Padding;

    // args
    UINT64 Arg1;
    UINT64 Arg2;
    UINT64 Arg3;

    UINT128 ExtendedArg1;
    UINT128 ExtendedArg2;
    UINT128 ExtendedArg3;
    UINT128 ExtendedArg4;
} HX_REQUEST_RESPONSE, *PHX_REQUEST_RESPONSE;

typedef struct _HX_ASYNC_INFO {
    HANDLE Handle;
    UINT64 SharedRegion[4];
} HX_ASYNC_INFO, *PHX_ASYNC_INFO;
#pragma pack(pop)

BOOL HxIsError(PHX_ERROR Error);
HX_ERROR HxErrorFromResult(PHX_RESULT Result);


HX_CALL HxCallGetStatus();
HX_CALL HxCallAuth();
HX_CALL HxCallOpenProcess();
HX_CALL HxCallOpenThread();
HX_CALL HxCallOpenToken();
HX_CALL HxCallCloseProcess();
HX_CALL HxCallCloseThread();
HX_CALL HxCallCloseToken();
HX_CALL HxCallGetProcessField();
HX_CALL HxCallGetTokenField();
HX_CALL HxCallGetThreadField();
HX_CALL HxCallSetProcessField();
HX_CALL HxCallSetThreadField();
HX_CALL HxCallMapMemory();
HX_CALL HxCallFreeMemory();
HX_CALL HxCallAllocateMemory();
HX_CALL HxCallProcessVmOp();
HX_CALL HxCallProtectVm();

INT HxpTrap(PHX_REQUEST_RESPONSE RequestResponse, PHX_ASYNC_INFO AsyncInfo);

#endif // !AAAAAAAAAA