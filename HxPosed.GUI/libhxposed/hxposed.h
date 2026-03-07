#pragma once
#include <Windows.h>

//  HXR Stands for HxRequest.
//  HXS Stands for HxResponse.

#define HX_ASYNC_BASE 0x20090000
#define HX_CALLBACK_RESERVED_OFFSET 0

typedef PVOID HX_THREAD;
typedef PVOID HX_PROCESS;
typedef PVOID HX_TOKEN;
typedef PVOID HX_RMD;
typedef PVOID HX_CALLBACK;

typedef enum _HX_OBJECT_TYPES {
    HxObHandle = 0,
    HxObProcess = 1,
    HxObThread = 2,
    HxObToken = 3,
    HxObRmd = 4,
    HxObRegKey = 5
} HX_OBJECT_TYPES;

typedef enum _HX_OBJECT_STATE {
    HxObCreated = 0,
    HxObModified = 1,
    HxObDeleted = 2
} HX_OBJECT_STATE;

typedef struct _HX_OBJECT_TYPE {
    HX_OBJECT_TYPES Type;
    PVOID Object;
} HX_OBJECT_TYPE;

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
    HxPoolNonPaged = 0,
    HxContiguousPhysical = 1
} HX_MEMORY_POOL;

typedef enum _HX_VM_OPERATION {
    HxVmRead = 0,
    HxVmWrite = 1
} HX_VM_OPERATION;

typedef enum _HX_PAGING_OBJECT {
    HxPml5 = 0,
    HxPml4 = 1,
    HxPdp = 2,
    HxPd = 3,
    HxPt = 4
} HX_PAGING_OBJECT;

typedef struct _HX_VIRTUAL_ADDRESS_FLAGS {
    UINT64 PhysicalOffset : 12;
    UINT64 PtIndex : 9;
    UINT64 PdIndex : 9;
    UINT64 PdpIndex : 9;
    UINT64 Pml4Index : 9;
    UINT64 Pml5Index : 9;
    UINT64 Sign : 7;
} HX_VIRTUAL_ADDRESS_FLAGS;

typedef union _HX_VIRTUAL_ADDRESS {
    PVOID Address;
    HX_VIRTUAL_ADDRESS_FLAGS Indices;
} HX_VIRTUAL_ADDRESS;

#pragma pack(push,1)
typedef struct _HX_PAGING_TYPE {
    HX_PAGING_OBJECT ObjectType;
    HX_VIRTUAL_ADDRESS Object;
} HX_PAGING_TYPE;
#pragma pack(pop)

typedef enum _HX_PAGING_OPERATION {
    HxPageOperationSet = 0,
    HxPageOperationGet = 1
} HX_PAGING_OPERATION;

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
///////////////////////////////////////////////////////////////////////////////////////// BEGIN MEMORY

typedef struct _HXS_GET_SET_PAGE_ATTRIBUTE {
    UINT64 TypeBits;
} HXS_GET_SET_PAGE_ATTRIBUTE, * PHXS_GET_SET_PAGE_ATTRIBUTE;

typedef struct _HXS_ALLOCATE_MEMORY {
    PVOID SystemVA;
} HXS_ALLOCATE_MEMORY, * PHXS_ALLOCATE_MEMORY;

typedef struct _HXS_TRANSLATE_MEMORY {
    UINT64 PhysicalAddress;
} HXS_TRANSLATE_MEMORY, *PHXS_TRANSLATE_MEMORY;


///////////////////////////////////////////////////////////////////////////////////////// END MEMORY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN PROCESS

typedef struct _HXS_GET_PROCESS_FIELD {
    HX_PROCESS_FIELD Field;
    union {
        UINT64 NtPathOffset;
        HX_PROCESS_PROTECTION Protection;
        HX_PROCESS_SIGNERS Signers;
        HX_PROCESS_MITIGATION_FLAGS MitigationFlags;
        UINT64 Token;
        UINT64 ThreadsOffset;
    };
} HXS_GET_PROCESS_FIELD, * PHXS_GET_PROCESS_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END PROCESS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN SECURITY

typedef struct _HXS_GET_TOKEN_FIELD {
    HX_TOKEN_FIELD Field;
    union {
        CHAR Name[8];
        UINT64 NameOffset;
        HX_TOKEN_TYPE Type;
        UINT32 Index;
        UINT32 Policy;
        HX_TOKEN_IMPERSONATION_LEVEL Level;
        HX_TOKEN_PRIVILEGES Privileges;
    };
} HXS_GET_TOKEN_FIELD, * PHXS_GET_TOKEN_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END SECURITY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN THREAD

typedef struct _HXS_GET_THREAD_FIELD {
    enum _HX_THREAD_FIELD Field;
    union {
        BOOL ImpersonationStatus;
        HX_TOKEN Token;
    };
} HXS_GET_THREAD_FIELD, * PHXS_GET_THREAD_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END THREAD
///////////////////////////////////////////////////////////////////////////////////////// BEGIN CALLBACKS

typedef struct _HXS_CALLBACK_INFORMATION {
    HX_OBJECT_TYPE ObjectType;
    HX_OBJECT_STATE ObjectState;
} HXS_CALLBACK_INFORMATION, *PHXS_CALLBACK_INFORMATION;

typedef struct _HXS_REGISTER_CALLBACK {
    HX_CALLBACK Object;
} HXS_REGISTER_CALLBACK, *PHXS_REGISTER_CALLBACK;

///////////////////////////////////////////////////////////////////////////////////////// END CALLBACKS


///////////////////////////////////////////////////////////////////////////////////////// BEGIN MEMORY


typedef struct _HXR_ALLOCATE_MEMORY {
    UINT32 Size;
    HX_MEMORY_POOL Pool;
} HXR_ALLOCATE_MEMORY, * PHXR_ALLOCATE_MEMORY;

typedef struct _HXR_FREE_MEMORY {
    HX_RMD Object;
} HXR_FREE_MEMORY, *PHXR_FREE_MEMORY;

typedef struct _HXR_MAP_VA_TO_PA {
    HX_PROCESS AddressSpace;
    HX_RMD MemoryDescriptor;
    PVOID MapAddress;
    HX_MAP_OPERATION Operation;
} HXR_MAP_VA_TO_PA, *PHXR_MAP_VA_TO_PA;

typedef struct _HXR_GET_SET_PAGE_ATTRIBUTE {
    HX_PROCESS AddressSpace;
    HX_PAGING_TYPE PagingType;
    UINT64 TypeBits;
    HX_PAGING_OPERATION Operation;
} HXR_GET_SET_PAGE_ATTRIBUTE, *PHXR_GET_SET_PAGE_ATTRIBUTE;

typedef struct _HXR_TRANSLATE_ADDRESS {
    UINT64 AddressSpace;
    UINT64 VirtualAddress;
} HXR_TRANSLATE_ADDRESS, *PHXR_TRANSLATE_ADDRESS;


///////////////////////////////////////////////////////////////////////////////////////// END MEMORY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN PROCESS

typedef struct _HXR_OPEN_PROCESS {
    UINT32 Id;
    HX_OPEN_TYPE OpenType;
} HXR_OPEN_PROCESS, * PHXR_OPEN_PROCESS;

typedef struct _HXR_CLOSE_PROCESS {
    HX_PROCESS Address;
    HX_OPEN_TYPE OpenType;
} HXR_CLOSE_PROCESS, * PHXR_CLOSE_PROCESS;

typedef struct _HXR_KILL_PROCESS {
    HX_PROCESS Address;
    UINT32 ExitCode;
} HXR_KILL_PROCESS, * PHXR_KILL_PROCESS;

typedef struct _HXR_GET_PROCESS_FIELD {
    HX_PROCESS Address;
    HXS_GET_PROCESS_FIELD Data;
} HXR_GET_PROCESS_FIELD, * PHXR_GET_PROCESS_FIELD;

typedef struct _HXR_SET_PROCESS_FIELD {
    HX_PROCESS Address;
    HXS_GET_PROCESS_FIELD Data;
} HXR_SET_PROCESS_FIELD, * PHXR_SET_PROCESS_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END PROCESS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN SECURITY

typedef struct _HXR_OPEN_TOKEN {
    HX_TOKEN Address;
    HX_OPEN_TYPE OpenType;
} HXR_OPEN_TOKEN, * PHXR_OPEN_TOKEN;

typedef struct _HXR_CLOSE_TOKEN {
    HX_TOKEN Address;
} HXR_CLOSE_TOKEN, * PHXR_CLOSE_TOKEN;

typedef struct _HXR_GET_TOKEN_FIELD {
    HX_TOKEN Address;
    HXS_GET_TOKEN_FIELD Data;
} HXR_GET_TOKEN_FIELD, * PHXR_GET_TOKEN_FIELD;

typedef struct _HXR_SET_TOKEN_FIELD {
    HX_TOKEN Address;
    HXS_GET_TOKEN_FIELD Data;
} HXR_SET_TOKEN_FIELD, * PHXR_SET_TOKEN_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END SECURITY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN THREAD

typedef struct _HXR_OPEN_THREAD {
    UINT32 Id;
    HX_OPEN_TYPE OpenType;
} HXR_OPEN_THREAD, * PHXR_OPEN_THREAD;

typedef struct _HXR_CLOSE_THREAD {
    HX_THREAD Address;
} HXR_CLOSE_THREAD, * PHXR_CLOSE_THREAD;

typedef struct _HXR_GET_THREAD_FIELD {
    HX_THREAD Address;
    HXS_GET_THREAD_FIELD Data;
} HXR_GET_THREAD_FIELD, * PHXR_GET_THREAD_FIELD;

typedef struct _HXR_SET_THREAD_FIELD {
    HX_THREAD Address;
    HXS_GET_THREAD_FIELD Data;
} HXR_SET_THREAD_FIELD, * PHXR_SET_THREAD_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END THREAD
///////////////////////////////////////////////////////////////////////////////////////// BEGIN CALLBACKS

typedef struct _HXR_REGISTER_CALLBACK {
    HX_OBJECT_TYPES ObjectType;
    HANDLE EventHandle;
} HXR_REGISTER_CALLBACK, *PHXR_REGISTER_CALLBACK;

typedef struct _HXR_UNREGISTER_CALLBACK {
    HX_CALLBACK Object;
} HXR_UNREGISTER_CALLBACK, *PHXR_UNREGISTER_CALLBACK;

///////////////////////////////////////////////////////////////////////////////////////// END CALLBACKS

typedef enum _HX_SERVICE_FUNCTION {
    /* General */
    HxSvcGetState = 0x00,

    /* Process Operations */
    HxSvcOpenProcess = 0x10,
    HxSvcCloseProcess = 0x11,
    HxSvcGetProcessField = 0x12,
    HxSvcSetProcessField = 0x13,

    /* Events */
    HxSvcRegisterNotifyEvent = 0x20,
    HxSvcUnregisterNotifyEvent = 0x21,

    /* Memory Management */
    HxSvcAllocateMemory = 0x30,
    HxSvcFreeMemory = 0x31,
    HxSvcGetSetPageAttribute = 0x32,
    HxSvcMapVaToPa = 0x33,
    HxSvcTranslateAddress = 0x34,

    /* Thread Operations */
    HxSvcOpenThread = 0x40,
    HxSvcCloseThread = 0x41,
    HxSvcGetThreadField = 0x42,
    HxSvcSetThreadField = 0x43,

    /* Token Operations */
    HxSvcOpenToken = 0x50,
    HxSvcCloseToken = 0x51,
    HxSvcGetTokenField = 0x53,
    HxSvcSetTokenField = 0x54,

    /* Privileged Operations */
    HxSvcMsrIo = 0x60,
    HxSvcExecutePrivilegedInstruction = 0x61,
    HxSvcInterProcessorInterrupt = 0x62
} HX_SERVICE_FUNCTION;

#pragma pack(push,1)
typedef struct _HX_RESULT {
    UINT32 ErrorCode;
    UINT32 ErrorReason;
} HX_RESULT, * PHX_RESULT;

typedef struct _HX_CALL {
    UINT64 ServiceFunction : 16;
    UINT64 IgnoreResult : 1;
    UINT64 ExtendedArgsPresent : 1;
    UINT64 Reserved : 46;
} HX_CALL, * PHX_CALL;

typedef struct _HX_REQUEST_RESPONSE {
    HX_CALL Call;
    HX_RESULT Result;

    UINT64 Arg1;
    UINT64 Arg2;
    UINT64 Arg3;

    UINT64 Padding;

    __uint128_t ExtendedArg1;
    __uint128_t ExtendedArg2;
    __uint128_t ExtendedArg3;
    __uint128_t ExtendedArg4;
} HX_REQUEST_RESPONSE, * PHX_REQUEST_RESPONSE;

#pragma pack(pop)

BOOL HxIsError(PHX_RESULT Error);

__declspec(dllexport) BOOL HxGetStatus(PHXS_STATUS Response);

__declspec(dllexport) BOOL HxpResponseFromRaw(PHX_REQUEST_RESPONSE RequestResponse, PVOID Response);
__declspec(dllexport) PHX_REQUEST_RESPONSE HxpRawFromRequest(HX_SERVICE_FUNCTION Function, PVOID Request);

__declspec(dllexport) INT HxpTrap(PHX_REQUEST_RESPONSE RequestResponse);

__declspec(dllexport) UINT32 HxReadAsyncResponseLength(UINT64 Offset);
__declspec(dllexport) PVOID HxReadAsyncResponseSlice(UINT64 Offset, PUINT32 Length);
__declspec(dllexport) PVOID HxReadAsyncResponseType(UINT64 Offset);