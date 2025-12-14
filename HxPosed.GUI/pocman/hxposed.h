#pragma once
#include <Windows.h>

typedef struct _UINT128 {
    UINT64 Low;
    UINT64 High;
} UINT128, * PUINT128;

///////////////////////////////////////////////////////////////////////////////////////// BEGIN SECURITY

typedef struct _HX_TOKEN_PRIVILEGES {
    UINT64 RESERVED : 2;
    UINT64 SeCreateTokenPrivilege : 1;
    UINT64 SeAssignPrimaryTokenPrivilege : 1;
    UINT64 SeLockMemoryPrivilege : 1;
    UINT64 SeIncreaseQuotaPrivilege : 1;
    UINT64 SeMachineAccountPrivilege : 1;
    UINT64 SeTcbPrivilege : 1;
    UINT64 SeSecurityPrivilege : 1;
    UINT64 SeTakeOwnershipPrivilege : 1;
    UINT64 SeLoadDriverPrivilege : 1;
    UINT64 SeSystemProfilePrivilege : 1;
    UINT64 SeSystemtimePrivilege : 1;
    UINT64 SeProfileSingleProcessPrivilege : 1;
    UINT64 SeIncreaseBasePriorityPrivilege : 1;
    UINT64 SeCreatePagefilePrivilege : 1;
    UINT64 SeCreatePermanentPrivilege : 1;
    UINT64 SeBackupPrivilege : 1;
    UINT64 SeRestorePrivilege : 1;
    UINT64 SeShutdownPrivilege : 1;
    UINT64 SeDebugPrivilege : 1;
    UINT64 SeAuditPrivilege : 1;
    UINT64 SeSystemEnvironmentPrivilege : 1;
    UINT64 SeChangeNotifyPrivilege : 1;
    UINT64 SeRemoteShutdownPrivilege : 1;
    UINT64 SeUndockPrivilege : 1;
    UINT64 SeSyncAgentPrivilege : 1;
    UINT64 SeEnableDelegationPrivilege : 1;
    UINT64 SeManageVolumePrivilege : 1;
    UINT64 SeImpersonatePrivilege : 1;
    UINT64 SeCreateGlobalPrivilege : 1;
    UINT64 SeTrustedCredManAccessPrivilege : 1;
    UINT64 SeRelabelPrivilege : 1;
    UINT64 SeIncreaseWorkingSetPrivilege : 1;
    UINT64 SeTimeZonePrivilege : 1;
    UINT64 SeCreateSymbolicLinkPrivilege : 1;
    UINT64 SeDelegateSessionUserImpersonatePrivilege : 1;
    UINT64 RESERVED2 : 27;
} HX_TOKEN_PRIVILEGES;


typedef enum _HX_TOKEN_IMPERSONATION_LEVEL {
    Anonymous = 0,
    Identification = 1,
    Impersonation = 2,
    Delegation = 3,
} HX_TOKEN_IMPERSONATION_LEVEL;

typedef enum _HX_TOKEN_TYPE {
    HxTokenPrimary,
    HxTokenImpersonation
} HX_TOKEN_TYPE;

///////////////////////////////////////////////////////////////////////////////////////// END SECURITY

///////////////////////////////////////////////////////////////////////////////////////// BEGIN PROCESS

typedef struct _HX_PROCESS_PROTECTION {
    union
    {
        UCHAR Level;                                                        //0x0
        struct
        {
            UCHAR Type : 3;                                                   //0x0
            UCHAR Audit : 1;                                                  //0x0
            UCHAR Signer : 4;                                                 //0x0
        };
    };
} HX_PROCESS_PROTECTION;

typedef union _HX_PROCESS_MITIGATION_FLAGS_2 {
    ULONG MitigationFlags2;                                             //0x754
    struct MitigationFlags2Values
    {
        ULONG EnableExportAddressFilter : 1;                              //0x754
        ULONG AuditExportAddressFilter : 1;                               //0x754
        ULONG EnableExportAddressFilterPlus : 1;                          //0x754
        ULONG AuditExportAddressFilterPlus : 1;                           //0x754
        ULONG EnableRopStackPivot : 1;                                    //0x754
        ULONG AuditRopStackPivot : 1;                                     //0x754
        ULONG EnableRopCallerCheck : 1;                                   //0x754
        ULONG AuditRopCallerCheck : 1;                                    //0x754
        ULONG EnableRopSimExec : 1;                                       //0x754
        ULONG AuditRopSimExec : 1;                                        //0x754
        ULONG EnableImportAddressFilter : 1;                              //0x754
        ULONG AuditImportAddressFilter : 1;                               //0x754
        ULONG DisablePageCombine : 1;                                     //0x754
        ULONG SpeculativeStoreBypassDisable : 1;                          //0x754
        ULONG CetUserShadowStacks : 1;                                    //0x754
        ULONG AuditCetUserShadowStacks : 1;                               //0x754
        ULONG AuditCetUserShadowStacksLogged : 1;                         //0x754
        ULONG UserCetSetContextIpValidation : 1;                          //0x754
        ULONG AuditUserCetSetContextIpValidation : 1;                     //0x754
        ULONG AuditUserCetSetContextIpValidationLogged : 1;               //0x754
        ULONG CetUserShadowStacksStrictMode : 1;                          //0x754
        ULONG BlockNonCetBinaries : 1;                                    //0x754
        ULONG BlockNonCetBinariesNonEhcont : 1;                           //0x754
        ULONG AuditBlockNonCetBinaries : 1;                               //0x754
        ULONG AuditBlockNonCetBinariesLogged : 1;                         //0x754
        ULONG XtendedControlFlowGuard_Deprecated : 1;                     //0x754
        ULONG AuditXtendedControlFlowGuard_Deprecated : 1;                //0x754
        ULONG PointerAuthUserIp : 1;                                      //0x754
        ULONG AuditPointerAuthUserIp : 1;                                 //0x754
        ULONG AuditPointerAuthUserIpLogged : 1;                           //0x754
        ULONG CetDynamicApisOutOfProcOnly : 1;                            //0x754
        ULONG UserCetSetContextIpValidationRelaxedMode : 1;               //0x754
    } _MitigationFlags2Values;                                           //0x754
} HX_PROCESS_MITIGATION_FLAGS_2;

typedef union _HX_PROCESS_MITIGATION_FLAGS_1 {
    ULONG MitigationFlags;                                              //0x750
    struct _MitigationFlagsValues
    {
        ULONG ControlFlowGuardEnabled : 1;                                //0x750
        ULONG ControlFlowGuardExportSuppressionEnabled : 1;               //0x750
        ULONG ControlFlowGuardStrict : 1;                                 //0x750
        ULONG DisallowStrippedImages : 1;                                 //0x750
        ULONG ForceRelocateImages : 1;                                    //0x750
        ULONG HighEntropyASLREnabled : 1;                                 //0x750
        ULONG StackRandomizationDisabled : 1;                             //0x750
        ULONG ExtensionPointDisable : 1;                                  //0x750
        ULONG DisableDynamicCode : 1;                                     //0x750
        ULONG DisableDynamicCodeAllowOptOut : 1;                          //0x750
        ULONG DisableDynamicCodeAllowRemoteDowngrade : 1;                 //0x750
        ULONG AuditDisableDynamicCode : 1;                                //0x750
        ULONG DisallowWin32kSystemCalls : 1;                              //0x750
        ULONG AuditDisallowWin32kSystemCalls : 1;                         //0x750
        ULONG EnableFilteredWin32kAPIs : 1;                               //0x750
        ULONG AuditFilteredWin32kAPIs : 1;                                //0x750
        ULONG DisableNonSystemFonts : 1;                                  //0x750
        ULONG AuditNonSystemFontLoading : 1;                              //0x750
        ULONG PreferSystem32Images : 1;                                   //0x750
        ULONG ProhibitRemoteImageMap : 1;                                 //0x750
        ULONG AuditProhibitRemoteImageMap : 1;                            //0x750
        ULONG ProhibitLowILImageMap : 1;                                  //0x750
        ULONG AuditProhibitLowILImageMap : 1;                             //0x750
        ULONG SignatureMitigationOptIn : 1;                               //0x750
        ULONG AuditBlockNonMicrosoftBinaries : 1;                         //0x750
        ULONG AuditBlockNonMicrosoftBinariesAllowStore : 1;               //0x750
        ULONG LoaderIntegrityContinuityEnabled : 1;                       //0x750
        ULONG AuditLoaderIntegrityContinuity : 1;                         //0x750
        ULONG EnableModuleTamperingProtection : 1;                        //0x750
        ULONG EnableModuleTamperingProtectionNoInherit : 1;               //0x750
        ULONG RestrictIndirectBranchPrediction : 1;                       //0x750
        ULONG IsolateSecurityDomain : 1;                                  //0x750
    } MitigationFlagsValues;                                            //0x750
} HX_PROCESS_MITIGATION_FLAGS_1;

typedef struct _HX_PROCESS_MITIGATION_FLAGS {
    HX_PROCESS_MITIGATION_FLAGS_1 First;
    HX_PROCESS_MITIGATION_FLAGS_2 Second;
} HX_PROCESS_MITIGATION_FLAGS;

typedef struct _HX_PROCESS_SIGNERS {
    UCHAR Level;
    UCHAR SectionLevel;
} HX_PROCESS_SIGNERS;

typedef enum _HX_PROCESS_PROTECTION_TYPE {
    HxPsProtTypeNone = 0,
    HxPsProtTypeLight = 1,
    HxPsProtTypeProtected = 2,
    HxPsProtTypeMax = 3,
} HX_PROCESS_PROTECTION_TYPE;

typedef enum _HX_PROCESS_PROTECTION_SIGNER {
    HxPsProtSigNone = 0,
    HxPsProtSigAuthenticode = 1,
    HxPsProtSigCodeGen = 2,
    HxPsProtSigAntiMalware = 3,
    HxPsProtSigLsa = 4,
    HxPsProtSigWindows = 5,
    HxPsProtSigWinTcb = 6,
    HxPsProtSigMax = 7,
} HX_PROCESS_PROTECTION_SIGNER;

typedef enum _HX_PROCESS_SIGNATURE_LEVEL {
    HxPsSigUnchecked = 0,
    HxPsSigUnsigned = 1,
    HxPsSigEnterprise = 2,
    HxPsSigCustom = 3,
    HxPsSigAuthenticode = 4,
    HxPsSigCustom2 = 5,
    HxPsSigStore = 6,
    HxPsSigAntiMalware = 7,
    HxPsSigMicrosoft = 8,
    HxPsSigCustom4 = 9,
    HxPsSigCustom5 = 10,
    HxPsSigDynamicCodeGen = 11,
    HxPsSigWindows = 12,
    HxPsSigWindowsPPL = 13,
    HxPsSigWindowsTcb = 14,
    HxPsSigCustom6 = 15,
} HX_PROCESS_SIGNATURE_LEVEL;

///////////////////////////////////////////////////////////////////////////////////////// END PROCESS

typedef enum _HXS_HYPERVISOR_STATUS {
    HxStatusUnknown = 0,
    SystemVirtualized = 1,
    SystemDeVirtualized = 2,
} HXS_HYPERVISOR_STATUS;

typedef struct _HXS_OPEN_OBJECT_RESPONSE {
    PVOID Address;
} HXS_OPEN_OBJECT_RESPONSE, * PHXS_OPEN_OBJECT_RESPONSE;

///////////////////////////////////////////////////////////////////////////////////////// BEGIN STATUS

typedef struct _HXS_STATUS {
    HXS_HYPERVISOR_STATUS Status;
    UINT64 Version;
} HXS_STATUS, * PHXS_STATUS;


///////////////////////////////////////////////////////////////////////////////////////// END STATUS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN AUTH

typedef struct _HXS_AUTH {
    UINT64 Permissions;
} HXS_AUTH, * PHXS_AUTH;

///////////////////////////////////////////////////////////////////////////////////////// END AUTH
///////////////////////////////////////////////////////////////////////////////////////// BEGIN MEMORY

typedef struct _HXS_RW_VM {
    SIZE_T BytesProcesseed;
} HXS_RW_VM, * PHXS_RW_VM;

typedef struct _HXS_ALLOCATE_MEMORY {
    PVOID Address;
    UINT32 BytesAllocated;
} HXS_ALLOCATE_MEMORY, * PHXS_ALLOCATE_MEMORY;

typedef struct _HXS_MAP_MEMORY {
    PVOID MappedAddress;
} HXS_MAP_MEMORY, * PHXS_MAP_MEMORY;

///////////////////////////////////////////////////////////////////////////////////////// END MEMORY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN PROCESS

typedef struct _HXS_GET_PROCESS_FIELD {
    enum _HX_PROCESS_FIELD Field;
    union _ProcessValues {
        struct _NtPath {
            UINT16 ByteLength;
        } NtPath;
        struct _Protection {
            struct _HX_PROCESS_PROTECTION Protection;
        } Protection;
        struct _Signers {
            struct _HX_PROCESS_SIGNERS Signers;
        } Signers;
        struct _Mitigation {
            struct _HX_PROCESS_MITIGATION_FLAGS MitigationFlags;
        } Mitigation;
        struct _Token {
            UINT64 Token;
        } Token;
    } ProcessValues;
} HXS_GET_PROCESS_FIELD, * PHXS_GET_PROCESS_FIELD;

typedef struct _HXS_GET_PROCESS_THREADS {
    UINT32 NumberOfThreads;
} HXS_GET_PROCESS_THREADS, * PHXS_GET_PROCESS_THREADS;

///////////////////////////////////////////////////////////////////////////////////////// END PROCESS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN SECURITY

typedef struct _HXS_GET_TOKEN_FIELD {
    enum _HX_TOKEN_FIELD Field;
    union _TokenValues {
        struct _SourceName {
            CHAR Name[8];
        } SourceName;
        struct _AccountName {
            UINT16 ByteLength;
        } AccountName;
        struct _Type {
            HX_TOKEN_TYPE Type;
        } Type;
        struct _IntegrityLevelIndex {
            UINT32 Index;
        } IntegrityLevelIndex;
        struct _MandatoryPolicy {
            UINT32 Policy;
        } MandatoryPolicy;
        struct _ImpersonationLevel {
            enum _HX_TOKEN_IMPERSONATION_LEVEL Level;
        } ImpersonationLevel;
        struct _Privileges {
            struct _HX_TOKEN_PRIVILEGES Privileges;
        } Privileges;
    } TokenValues;
} HXS_GET_TOKEN_FIELD, * PHXS_GET_TOKEN_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END SECURITY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN THREAD

typedef struct _HXS_GET_THREAD_FIELD {
    enum _HX_THREAD_FIELD Field;
    union _ThreadValues {
        struct _ActiveImpersonationInfo {
            BOOL Status;
        } ActiveImpersonationInfo;
        struct _AdjustClientToken {
            PVOID Token;
        } AdjustedClientToken;
    } ThreadValues;
} HXS_GET_THREAD_FIELD, * PHXS_GET_THREAD_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END THREAD

typedef enum _HX_THREAD_FIELD {
    HxThreadFieldUnknown = 0,
    HxThreadFieldActiveImpersonationInfo = 1,
    HxThreadFieldAdjustedClientToken = 2,
} HX_THREAD_FIELD;

typedef enum _HX_TOKEN_FIELD {
    HxTokenFieldUnknown = 0,
    HxTokenFieldSourceName = 1,
    HxTokenFieldAccountName = 2,
    HxTokenFieldType = 3,
    HxTokenFieldIntegrityLevelIndex = 4,
    HxTokenFieldMandatoryPolicy = 5,
    HxTokenFieldImpersonationLevel = 6,
    HxTokenFieldPresentPrivileges = 7,
    HxTokenFieldEnabledPrivileges = 8,
    HxTokenFieldEnabledByDefaultPrivileges = 9,
} HX_TOKEN_FIELD;

typedef enum _HX_PROCESS_FIELD {
    HxProcFieldUnknown = 0,
    HxProcFieldNtPath = 1,
    HxProcFieldProtection = 2,
    HxProcFieldSigners = 3,
    HxProcFieldMitigationFlags = 4,
    HxProcFieldToken = 5,
} HX_PROCESS_FIELD;

typedef enum _HX_OPEN_TYPE {
    HxOpenHandle = 0,
    HxOpenHypervisor = 1
} HX_OPEN_TYPE;

typedef enum _HX_MAP_OPERATION {
    HxMemMap = 0,
    HxMemUnMap = 1
} HX_MAP_OPERATION, * PHX_MAP_OPERATION;

typedef enum _HX_MEMORY_POOL {
    HxPoolNonPaged = 0
} HX_MEMORY_POOL;

typedef enum _HX_VM_OPERATION {
    HxVmRead = 0,
    HxVmWrite = 1
} HX_VM_OPERATION;

///////////////////////////////////////////////////////////////////////////////////////// BEGIN AUTH

typedef struct _HXR_AUTH {
    struct _UINT128 Guid;
    UINT64 Permissions;
} HXR_AUTH, * PHXR_AUTH;

///////////////////////////////////////////////////////////////////////////////////////// END AUTH
///////////////////////////////////////////////////////////////////////////////////////// BEGIN MEMORY

typedef struct _HXR_RW_VM {
    PVOID ProcessAddress;
    PVOID Address;
    SIZE_T Count;
    PVOID Output;
    SIZE_T OutputSize;
    enum _HX_VM_OPERATION Operation;
} HXR_RW_VM, * PHXR_RW_VM;

typedef struct _HXR_PROTECT_VM {
    PVOID ProcessAddress;
    PVOID Address;
    UINT32 Protection;
} HXR_PROTECT_VM, * PHXR_PROTECT_VM;

typedef struct _HXR_ALLOCATE_MEMORY {
    UINT32 Size;
    UINT32 Reserved;
    enum _HX_MEMORY_POOL Pool;
} HXR_ALLOCATE_MEMORY, * PHXR_ALLOCATE_MEMORY;

typedef struct _HXR_MAP_MEMORY {
    PVOID Mdl;
    PVOID MapAddress;
    enum _HX_MAP_OPERATION Operation;
} HXR_MAP_MEMORY, * PHXR_MAP_MEMORY;

typedef struct _HXR_FREE_MEMORY {
    PVOID Mdl;
} HXR_FREE_MEMORY, * PHXR_FREE_MEMORY;

///////////////////////////////////////////////////////////////////////////////////////// END MEMORY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN PROCESS

typedef struct _HXR_OPEN_PROCESS {
    UINT32 Id;
    enum _HX_OPEN_TYPE OpenType;
} HXR_OPEN_PROCESS, * PHXR_OPEN_PROCESS;

typedef struct _HXR_CLOSE_PROCESS {
    PVOID Address;
} HXR_CLOSE_PROCESS, * PHXR_CLOSE_PROCESS;

typedef struct _HXR_KILL_PROCESS {
    PVOID Address;
    UINT32 ExitCode;
} HXR_KILL_PROCESS, * PHXR_KILL_PROCESS;

typedef struct _HXR_GET_PROCESS_FIELD {
    PVOID Address;
    enum _HX_PROCESS_FIELD Field;
    PVOID Data;
    SIZE_T DataLen;
} HXR_GET_PROCESS_FIELD, * PHXR_GET_PROCESS_FIELD;

typedef struct _HXR_SET_PROCESS_FIELD {
    PVOID Address;
    enum _HX_PROCESS_FIELD Field;
    PVOID Data;
    SIZE_T DataLen;
} HXR_SET_PROCESS_FIELD, * PHXR_SET_PROCESS_FIELD;

typedef struct _HXR_GET_PROCESS_THREADS {
    PVOID Address;
    PVOID Data;
    SIZE_T DataLen;
} HXR_GET_PROCESS_THREADS, * PHXR_GET_PROCESS_THREADS;

///////////////////////////////////////////////////////////////////////////////////////// END PROCESS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN SECURITY

typedef struct _HXR_OPEN_TOKEN {
    PVOID Address;
    enum _HX_OPEN_TYPE OpenType;
} HXR_OPEN_TOKEN, * PHXR_OPEN_TOKEN;

typedef struct _HXR_CLOSE_TOKEN {
    PVOID Address;
} HXR_CLOSE_TOKEN, * PHXR_CLOSE_TOKEN;

typedef struct _HXR_GET_TOKEN_FIELD {
    PVOID Address;
    enum _HX_TOKEN_FIELD Field;
    PVOID Data;
    SIZE_T DataLen;
} HXR_GET_TOKEN_FIELD, * PHXR_GET_TOKEN_FIELD;

typedef struct _HXR_SET_TOKEN_FIELD {
    PVOID Address;
    enum _HX_TOKEN_FIELD Field;
    PVOID Data;
    SIZE_T DataLen;
} HXR_SET_TOKEN_FIELD, * PHXR_SET_TOKEN_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END SECURITY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN THREAD

typedef struct _HXR_OPEN_THREAD {
    UINT32 Id;
    enum _HX_OPEN_TYPE OpenType;
} HXR_OPEN_THREAD, * PHXR_OPEN_THREAD;

typedef struct _HXR_CLOSE_THREAD {
    PVOID Address;
} HXR_CLOSE_THREAD, * PHXR_CLOSE_THREAD;

typedef struct _HXR_GET_THREAD_FIELD {
    PVOID Address;
    enum _HX_THREAD_FIELD Field;
    PVOID Data;
    SIZE_T DataLen;
} HXR_GET_THREAD_FIELD, * PHXR_GET_THREAD_FIELD;

typedef struct _HXR_SET_THREAD_FIELD {
    PVOID Address;
    enum _HX_THREAD_FIELD Field;
    PVOID Data;
    SIZE_T DataLen;
} HXR_SET_THREAD_FIELD, * PHXR_SET_THREAD_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END THREAD

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
    enum _HX_ERROR_SOURCE ErrorSource;
    UINT16 ErrorCode;
    UINT16 ErrorReason;
} HX_ERROR, * PHX_ERROR;

#pragma pack(push,1)
typedef struct _HX_RESULT {
    enum _HX_SERVICE_FUNCTION ServiceFunction : 16;
    UINT32 ErrorSource : 2;
    enum _HX_ERROR_CODE ErrorCode : 3;
    UINT32 Reserved : 11;
} HX_RESULT, * PHX_RESULT;

typedef struct _HX_CALL {
    enum _HX_SERVICE_FUNCTION ServiceFunction : 16;
    BOOL IsFast : 1;
    BOOL IgnoreResult : 1;
    BOOL YieldExecution : 1;
    BOOL IsAsync : 1;
    BOOL ExtendedArgsPresent : 1;
    UINT32 Reserved : 10;
} HX_CALL, * PHX_CALL;

typedef struct _HX_REQUEST_RESPONSE {
    HX_CALL Call;
    HX_RESULT Result;

    // args
    UINT64 Arg1;
    UINT64 Arg2;
    UINT64 Arg3;

    UINT128 ExtendedArg1;
    UINT128 ExtendedArg2;
    UINT128 ExtendedArg3;
    UINT128 ExtendedArg4;
} HX_REQUEST_RESPONSE, * PHX_REQUEST_RESPONSE;

typedef struct _HX_ASYNC_INFO {
    HANDLE Handle;
    HX_RESULT Result;
    UINT32 Pad0;

    UINT64 Arg1;
    UINT64 Arg2;
    UINT64 Arg3;
} HX_ASYNC_INFO, * PHX_ASYNC_INFO;
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
HX_CALL HxCallGetProcessThreads();


__declspec(dllexport) HX_ERROR HxGetStatus(PHXS_STATUS Response);
__declspec(dllexport) HX_ERROR HxpResponseFromAsync(PHX_ASYNC_INFO Async, PVOID Result);
__declspec(dllexport) HANDLE HxpCreateEventHandle();

__declspec(dllexport) HX_ERROR HxpResponseFromAsync(PHX_ASYNC_INFO Async, PVOID Result);
__declspec(dllexport) HX_ERROR HxpResponseFromRaw(PHX_REQUEST_RESPONSE RequestResponse, PVOID Response);
__declspec(dllexport) PHX_REQUEST_RESPONSE HxpRawFromRequest(HX_SERVICE_FUNCTION Function, PVOID Request);

__declspec(dllexport) INT HxpTrap(PHX_REQUEST_RESPONSE RequestResponse, PHX_ASYNC_INFO AsyncInfo);