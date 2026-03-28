#pragma once
#ifndef HXPOSED
#define HXPOSED

#include <Windows.h>

// incredible
#pragma comment(linker, "/EXPORT:HxpTrap")

//  HXR Stands for HxRequest.
//  HXS Stands for HxResponse.

#define HX_ASYNC_BASE 0x20090000
#define HX_CALLBACK_RESERVED_OFFSET 0
#define DLL_EXPORT __declspec(dllexport)

typedef UINT64 HX_OBJECT;
typedef HX_OBJECT *PHX_OBJECT;

typedef HX_OBJECT HX_THREAD;
typedef HX_OBJECT HX_PROCESS;
typedef HX_OBJECT HX_TOKEN;
typedef HX_TOKEN *PHX_TOKEN;
typedef HX_OBJECT HX_RMD;
typedef HX_RMD *PHX_RMD;
typedef HX_OBJECT HX_CALLBACK;
typedef HX_CALLBACK *PHX_CALLBACK;

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
    HX_OBJECT Object;
} HX_OBJECT_TYPE;

typedef struct _HXR_OPEN_OBJECT {
    UINT64 AddressOrId;
} HXR_OPEN_OBJECT, *PHXR_OPEN_OBJECT;

typedef struct _HXR_CLOSE_OBJECT {
    HX_OBJECT Address;
} HXR_CLOSE_OBJECT, *PHXR_CLOSE_OBJECT;

///////////////////////////////////////////////////////////////////////////////////////// BEGIN SECURITY

typedef union _HX_TOKEN_PRIVILEGES {
    UINT64 All;
    union {
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
    };
} HX_TOKEN_PRIVILEGES, *PHX_TOKEN_PRIVILEGES;


typedef enum _HX_TOKEN_IMPERSONATION_LEVEL {
    Anonymous = 0,
    Identification = 1,
    Impersonation = 2,
    Delegation = 3,
} HX_TOKEN_IMPERSONATION_LEVEL, *PHX_TOKEN_IMPERSONATION_LEVEL;

typedef enum _HX_TOKEN_TYPE {
    HxTokenPrimary = 0,
    HxTokenImpersonation = 1
} HX_TOKEN_TYPE, *PHX_TOKEN_TYPE;

///////////////////////////////////////////////////////////////////////////////////////// END SECURITY

///////////////////////////////////////////////////////////////////////////////////////// BEGIN PROCESS

typedef UINT64 HX_THREAD_FIELD;
enum _HX_THREAD_FIELD {
    HxThreadFieldActiveImpersonationInfo = 1,
    HxThreadFieldAdjustedClientToken = 2,
};

typedef UINT64 HX_TOKEN_FIELD;
enum _HX_TOKEN_FIELD  {
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
};

typedef UINT64 HX_PROCESS_FIELD;
enum _HX_PROCESS_FIELD {
    HxProcessFieldUnknown = 0,
    HxProcessFieldNtPath = 1,
    HxProcessFieldProtection = 2,
    HxProcessFieldSigners = 3,
    HxProcessFieldMitigation = 4,
    HxProcessFieldToken = 5,
    HxProcessFieldThreads = 6,
    HxProcessFieldDirectoryTableBase = 7,
    HxProcessFieldUserDirectoryTableBase = 8,
};

typedef UINT64 HX_MAP_OPERATION;
enum _HX_MAP_OPERATION {
    HxMemMap = 0,
    HxMemUnMap = 1
};

typedef UINT64 HX_MEMORY_POOL;
enum _HX_MEMORY_POOL {
    HxPoolNonPaged = 0,
    HxContiguousPhysical = 1
};


typedef UINT64 HX_PAGING_OBJECT;
enum _HX_PAGING_OBJECT {
    HxPml5 = 0,
    HxPml4 = 1,
    HxPdp = 2,
    HxPd = 3,
    HxPt = 4,
};

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

typedef struct _HX_PAGING_TYPE {
    HX_PAGING_OBJECT ObjectType;
    HX_VIRTUAL_ADDRESS Object;
} HX_PAGING_TYPE;

typedef UINT64 HX_PAGING_OPERATION;
enum _HX_PAGING_OPERATION {
    HxPageOperationSet = 0,
    HxPageOperationGet = 1
};

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
} HX_PROCESS_PROTECTION, *PHX_PROCESS_PROTECTION;

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
} HX_PROCESS_MITIGATION_FLAGS, *PHX_PROCESS_MITIGATION_FLAGS;

typedef struct _HX_PROCESS_SIGNERS {
    UCHAR Level;
    UCHAR SectionLevel;
} HX_PROCESS_SIGNERS, *PHX_PROCESS_SIGNERS;

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
    HX_OBJECT_TYPE Object;
} HXS_OPEN_OBJECT_RESPONSE, * PHXS_OPEN_OBJECT_RESPONSE;

///////////////////////////////////////////////////////////////////////////////////////// BEGIN STATUS

typedef struct _HXS_STATUS {
    HXS_HYPERVISOR_STATUS Status;
    UINT32 _PAD;
    UINT32 Version;
    UINT32 _PAD2;
} HXS_STATUS, * PHXS_STATUS;

///////////////////////////////////////////////////////////////////////////////////////// END STATUS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN MEMORY

typedef struct _HXS_GET_SET_PAGE_ATTRIBUTE {
    UINT64 TypeBits;
} HXS_GET_SET_PAGE_ATTRIBUTE, * PHXS_GET_SET_PAGE_ATTRIBUTE;

typedef struct _HXS_ALLOCATE_MEMORY {
    HX_RMD RawMemoryDescriptor;
} HXS_ALLOCATE_MEMORY, * PHXS_ALLOCATE_MEMORY;

typedef struct _HXS_DESCRIBE_MEMORY {
    HX_RMD RawMemoryDescriptor;
} HXS_DESCRIBE_MEMORY, * PHXS_DESCRIBE_MEMORY;

typedef struct _HXS_TRANSLATE_ADDRESS {
    UINT64 PhysicalAddress;
} HXS_TRANSLATE_ADDRESS, *PHXS_TRANSLATE_ADDRESS;


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
        UINT64 DirectoryTableBase;
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
        UINT32 IntegrityIndex;
        UINT32 Policy;
        HX_TOKEN_IMPERSONATION_LEVEL Impersonationlevel;
        HX_TOKEN_PRIVILEGES Privileges;
    };
} HXS_GET_TOKEN_FIELD, * PHXS_GET_TOKEN_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END SECURITY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN THREAD

typedef struct _HXS_GET_THREAD_FIELD {
    HX_THREAD_FIELD Field;
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
///////////////////////////////////////////////////////////////////////////////////////// BEGIN CPU/IO

typedef UINT64 HX_MSR_OPERATION;
enum {
    HxMsrRead = 0,
    HxMsrWrite = 1
};

typedef UINT64 HX_PRIVILEGED_INSTRUCTION;
enum {
    HxPiHlt = 0,
    HxPiMovToCr8 = 1,
    HxPiMovToCr3 = 2,
    HxPiMovFromCr8 = 3,
    HxPiMovFromCr3 = 4,
    HxPiLgdt = 5,
    HxPiLidt = 6,
    HxPiSgdt = 7,
    HxPiSidt = 8,
    HxPiCli = 9,
    HxPiSti = 10
};

typedef struct _HXS_EXECUTE_PRIVILEGED {
    HX_PRIVILEGED_INSTRUCTION Instruction;
    union {
        UINT64 Cr3;
        UINT64 Cr8;
        UINT64 Gdt;
        UINT64 Idt;
    };
} HXS_EXECUTE_PRIVILEGED, * PHXS_EXECUTE_PRIVILEGED;

typedef struct _HXS_MSR_OPERATION {
    UINT64 Msr;
} HXS_MSR_OPERATION, *PHXS_MSR_OPERATION;

///////////////////////////////////////////////////////////////////////////////////////// END CALLBACKS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN HANDLE

typedef struct _HXS_GET_HANDLE_OBJECT {
    UINT64 Object;
    UINT32 GrantedAccess;
    UINT32 _PAD;
} HXS_GET_HANDLE_OBJECT, *PHXS_GET_HANDLE_OBJECT;

///////////////////////////////////////////////////////////////////////////////////////// END HANDLE



///////////////////////////////////////////////////////////////////////////////////////// BEGIN MEMORY

typedef struct _HXR_ALLOCATE_MEMORY {
    UINT32 Size;
    UINT32 _PAD;
    HX_MEMORY_POOL Pool;
} HXR_ALLOCATE_MEMORY, * PHXR_ALLOCATE_MEMORY;

typedef struct _HXR_FREE_MEMORY {
    HX_RMD Object;
} HXR_FREE_MEMORY, *PHXR_FREE_MEMORY;

typedef struct _HXR_MAP_RAW_MEMORY_DESCRIPTOR {
    HX_RMD MemoryDescriptor;
    HX_PROCESS AddressSpace;
    PVOID MapAddress;
    UINT64 _PAD;
    HX_MAP_OPERATION Operation;
} HXR_MAP_RAW_MEMORY_DESCRIPTOR, *PHXR_MAP_RAW_MEMORY_DESCRIPTOR;

typedef struct _HXR_GET_SET_PAGE_ATTRIBUTE {
    HX_PROCESS AddressSpace;
    HX_PAGING_OPERATION Operation;
    UINT64 TypeBits;
    UINT64 _PAD;
    HX_PAGING_TYPE PagingType;
} HXR_GET_SET_PAGE_ATTRIBUTE, *PHXR_GET_SET_PAGE_ATTRIBUTE;

typedef struct _HXR_TRANSLATE_ADDRESS {
    HX_PROCESS AddressSpace;
    UINT64 VirtualAddress;
} HXR_TRANSLATE_ADDRESS, *PHXR_TRANSLATE_ADDRESS;

typedef struct _HXR_DESCRIBE_MEMORY {
    UINT64 PhysicalAddress;
    UINT32 Size;
} HXR_DESCRIBE_MEMORY, *PHXR_DESCRIBE_MEMORY;


///////////////////////////////////////////////////////////////////////////////////////// END MEMORY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN PROCESS

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
    HX_OBJECT_TYPE ObjectType;
    HANDLE EventHandle;
    PVOID Memory;
} HXR_REGISTER_CALLBACK, *PHXR_REGISTER_CALLBACK;

typedef struct _HXR_UNREGISTER_CALLBACK {
    HX_CALLBACK Object;
} HXR_UNREGISTER_CALLBACK, *PHXR_UNREGISTER_CALLBACK;

///////////////////////////////////////////////////////////////////////////////////////// END CALLBACKS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN CPU/IO

typedef struct _HXR_EXECUTE_PRIVILEGED {
    HX_PRIVILEGED_INSTRUCTION Instruction;
    union {
        UINT64 Cr3;
        UINT64 Cr8;
        UINT64 Gdt;
        UINT64 Idt;
    };
} HXR_EXECUTE_PRIVILEGED, *PHXR_EXECUTE_PRIVILEGED;

typedef struct _HXR_MSR_OPERATION {
    UINT64 Msr;
    UINT64 Value;
    HX_MSR_OPERATION Operation;
} HXR_MSR_OPERATION, *PHXR_MSR_OPERATION;
 
///////////////////////////////////////////////////////////////////////////////////////// END CPU/IO
///////////////////////////////////////////////////////////////////////////////////////// BEGIN HANDLE

typedef struct _HXR_UPGRADE_HANDLE {
    UINT64 Handle;
    HX_PROCESS Process;
    UINT64 AccessMask;
} HXR_UPGRADE_HANDLE, *PHXR_UPGRADE_HANDLE;

typedef struct _HXR_SWAP_HANDLE_OBJECT {
    UINT64 Handle;
    HX_PROCESS Process;
    UINT64 NewObject;
} HXR_SWAP_HANDLE_OBJECT, *PHXR_SWAP_HANDLE_OBJECT;

typedef struct _HXR_GET_HANDLE_OBJECT {
    UINT64 Handle;
    HX_PROCESS Process;
} HXR_GET_HANDLE_OBJECT, * PHXR_GET_HANDLE_OBJECT;

///////////////////////////////////////////////////////////////////////////////////////// END HANDLE

typedef enum _HX_SERVICE_FUNCTION {
    HxSvcGetState = 0x0,

    HxSvcOpenProcess = 0x10,
    HxSvcCloseProcess = 0x11,
    HxSvcGetProcessField = 0x12,
    HxSvcSetProcessField = 0x13,

    HxSvcRegisterNotifyEvent = 0x20,
    HxSvcUnregisterNotifyEvent = 0x21,

    HxSvcAllocateMemory = 0x30,
    HxSvcFreeMemory = 0x31,
    HxSvcGetSetPageAttribute = 0x32,
    HxSvcMapRawMemoryDescriptor = 0x33,
    HxSvcTranslateAddress = 0x34,
    HxSvcDescribeMemory = 0x35,

    HxSvcOpenThread = 0x40,
    HxSvcCloseThread = 0x41,
    HxSvcGetThreadField = 0x42,
    HxSvcSetThreadField = 0x43,

    HxSvcOpenToken = 0x50,
    HxSvcCloseToken = 0x51,
    HxSvcGetTokenField = 0x53,
    HxSvcSetTokenField = 0x54,

    HxSvcMsrIo = 0x60,
    HxSvcExecutePrivilegedInstruction = 0x61,
    HxSvcInterProcessorInterrupt = 0x62,

    HxSvcUpgradeHandle = 0x70,
    HxSvcGetHandleObject = 0x71,
    HxSvcSwapHandleObject = 0x72
} HX_SERVICE_FUNCTION;

typedef enum _HX_ERROR_CODE{
    HxErrSuccess = 0,
    HxErrNotAllowed = 1,
    HxErrNotFound = 2,
    HxErrInvalidParameters = 3,
    HxErrNtError = 4,
    HxErrTimedOut = 5,
    HxErrHvNotLoaded = 6
} HX_ERROR_CODE;

typedef enum _HX_NOT_ALLOWED_REASON {
   HxErrReasonLockHeld = 2,
   HxErrReasonPageNotPresent = 3,
   HxErrReasonMappingsExist = 4,
   HxErrReasonAccessViolation = 5
} HX_NOT_ALLOWED_REASON;

typedef enum _HX_NOT_FOUND_REASON {
    HxErrReasonProcess = 1,
    HxErrReasonMdl = 3,
    HxErrReasonThread = 4,
    HxErrReasonFunction = 5,
    HxErrReasonToken = 6,
    HxErrReasonCallback = 7,
    HxErrReasonEvent = 9,
    HxErrReasonField = 10,
    HxErrReasonHandle = 11
} HX_NOT_FOUND_REASON;

#pragma pack(push,1)
typedef struct _HX_RESULT {
    HX_ERROR_CODE ErrorCode;
    union {
        enum _HX_NOT_ALLOWED_REASON NotAllowedReason;
        enum _HX_NOT_FOUND_REASON NotFoundReason;
        UINT32 NtStatus;
        UINT32 Parameter;
    };
} HX_RESULT, * PHX_RESULT;

typedef struct _HX_CALL {
    UINT64 ServiceFunction : 16;
    UINT64 IgnoreResult : 1;
    UINT64 ExtendedArgsPresent : 1;
    UINT64 Reserved : 46;
} HX_CALL, * PHX_CALL;

typedef struct _HX_REQUEST_RESPONSE {
    HX_CALL Call;                       // 0
    HX_RESULT Result;                   // 8

    union {
        struct {
            UINT64 Arg1;                // 16
            UINT64 Arg2;                // 24
            UINT64 Arg3;                // 32

            UINT64 Padding;             // 40

            __uint128_t ExtendedArg1;   // 48
            __uint128_t ExtendedArg2;   // 64
            __uint128_t ExtendedArg3;   // 80
            __uint128_t ExtendedArg4;   // 96
        };                              // total 112

        HXS_STATUS StatusResponse;
        HXS_OPEN_OBJECT_RESPONSE OpenObjectResponse;

        HXS_GET_SET_PAGE_ATTRIBUTE GetSetPageAttributeResponse;
        HXS_ALLOCATE_MEMORY AllocateMemoryResponse;
        HXS_DESCRIBE_MEMORY DescribeMemoryResponse;
        HXS_TRANSLATE_ADDRESS TranslateAddressResponse;

        HXS_REGISTER_CALLBACK RegisterCallbackResponse;

        HXS_GET_PROCESS_FIELD GetProcessFieldResponse;
        HXS_GET_TOKEN_FIELD GetTokenFieldResponse;
        HXS_GET_THREAD_FIELD GetThreadFieldResponse;

        HXS_MSR_OPERATION MsrIoResponse;
        HXS_EXECUTE_PRIVILEGED ExecutePrivilegedInstructionResponse;

        HXS_GET_HANDLE_OBJECT GetHandleObjectResponse;


        HXR_OPEN_OBJECT OpenObjectRequest;
        HXR_CLOSE_OBJECT CloseObjectRequest;

        HXR_ALLOCATE_MEMORY AllocateMemoryRequest;
        HXR_FREE_MEMORY FreeMemoryRequest;
        HXR_MAP_RAW_MEMORY_DESCRIPTOR MapRawMemoryDescriptorRequest;
        HXR_DESCRIBE_MEMORY DescribeMemoryRequest;
        HXR_TRANSLATE_ADDRESS TranslateAddressRequest;
        HXR_GET_SET_PAGE_ATTRIBUTE GetSetPageAttributeRequest;

        HXR_REGISTER_CALLBACK RegisterCallbackRequest;
        HXR_UNREGISTER_CALLBACK UnregisterCallbackRequest;

        HXR_GET_PROCESS_FIELD GetProcessFieldRequest;
        HXR_SET_PROCESS_FIELD SetProcessFieldRequest;

        HXR_GET_TOKEN_FIELD GetTokenFieldRequest;
        HXR_SET_TOKEN_FIELD SetTokenFieldRequest;

        HXR_GET_THREAD_FIELD GetThreadFieldRequest;
        HXR_SET_THREAD_FIELD SetThreadFieldRequest;

        HXR_MSR_OPERATION MsrIoRequest;
        HXR_EXECUTE_PRIVILEGED ExecutePrivilegedInstructionRequest;

        HXR_UPGRADE_HANDLE UpgradeHandleRequest;
        HXR_SWAP_HANDLE_OBJECT SwapHandleObjectRequest;
        HXR_GET_HANDLE_OBJECT GetHandleObjectRequest;
    };
} HX_REQUEST_RESPONSE, * PHX_REQUEST_RESPONSE;

#pragma pack(pop)

DLL_EXPORT HX_RESULT HxpTrap(PHX_REQUEST_RESPONSE RequestResponse);

DLL_EXPORT BOOL HxGetStatus(PHXS_STATUS Response);
DLL_EXPORT HX_RESULT HxCloseObject(HX_SERVICE_FUNCTION Function, HX_OBJECT Object);
DLL_EXPORT HX_RESULT HxOpenObject(HX_SERVICE_FUNCTION Function, PVOID AddrOrId, PHX_OBJECT Object);

#define GENERATE_HX_GET_FUNC_SIGN(x,y,z) \
    DLL_EXPORT HX_RESULT HxGet##x##y(HX_OBJECT x, z y);
#define GENERATE_HX_SET_FUNC_SIGN(x,y,z) \
    DLL_EXPORT HX_RESULT HxSet##x##y(HX_OBJECT x, z y);

#define GENERATE_HX_FUNC_SIGN(x, y, z) \
    GENERATE_HX_GET_FUNC_SIGN(x,y,z) \
    GENERATE_HX_SET_FUNC_SIGN(x,y,z)
    
GENERATE_HX_FUNC_SIGN(Process, Protection, PHX_PROCESS_PROTECTION)
GENERATE_HX_FUNC_SIGN(Process, Mitigation, PHX_PROCESS_MITIGATION_FLAGS)
GENERATE_HX_FUNC_SIGN(Process, Signers, PHX_PROCESS_SIGNERS)
GENERATE_HX_FUNC_SIGN(Process, Token, PUINT64)
GENERATE_HX_FUNC_SIGN(Process, DirectoryTableBase, PUINT64)
DLL_EXPORT HX_RESULT HxGetProcessNtPath(HX_PROCESS Process, PWCHAR Name, PSIZE_T CharCount);
DLL_EXPORT HX_RESULT HxGetProcessThreads(HX_PROCESS Process, PUINT32 Threads, PSIZE_T Count);

GENERATE_HX_FUNC_SIGN(Token, SourceName, PCHAR)
GENERATE_HX_FUNC_SIGN(Token, Type, PHX_TOKEN_TYPE)
GENERATE_HX_FUNC_SIGN(Token, IntegrityLevelIndex, PUINT32)
GENERATE_HX_FUNC_SIGN(Token, MandatoryPolicy, PUINT32)
GENERATE_HX_FUNC_SIGN(Token, ImpersonationLevel, PHX_TOKEN_IMPERSONATION_LEVEL)
GENERATE_HX_FUNC_SIGN(Token, PresentPrivileges, PHX_TOKEN_PRIVILEGES)
GENERATE_HX_FUNC_SIGN(Token, EnabledPrivileges, PHX_TOKEN_PRIVILEGES)
GENERATE_HX_FUNC_SIGN(Token, EnabledByDefaultPrivileges, PHX_TOKEN_PRIVILEGES)
DLL_EXPORT HX_RESULT HxGetTokenAccountName(HX_PROCESS Process, PWCHAR Name, PSIZE_T CharCount);

GENERATE_HX_FUNC_SIGN(Thread, ActiveImpersonationInfo, PBOOL)
GENERATE_HX_FUNC_SIGN(Thread, AdjustedClientToken, PHX_TOKEN)

#undef GENERATE_HX_GET_FUNC_SIGN
#undef GENERATE_HX_SET_FUNC_SIGN
#undef GENERATE_HX_FUNC_SIGN

DLL_EXPORT HX_RESULT HxReadMsr(UINT64 Msr, PUINT64 Value);
DLL_EXPORT HX_RESULT HxWriteMsr(UINT64 Msr, UINT64 Value);
DLL_EXPORT HX_RESULT HxExecPrivileged(HX_PRIVILEGED_INSTRUCTION Instruction, PUINT64 Result);

DLL_EXPORT HX_RESULT HxUpgradeHandle(UINT64 Handle, HX_PROCESS Process, UINT32 AccessMask);
DLL_EXPORT HX_RESULT HxSwapHandleObject(UINT64 Handle, HX_PROCESS Process, HX_OBJECT NewObject);
DLL_EXPORT HX_RESULT HxGetHandleObject(UINT64 Handle, HX_PROCESS Process, PHX_OBJECT Object, PUINT32 GrantedAccess);

DLL_EXPORT HX_RESULT HxAllocateMemory(HX_MEMORY_POOL Pool, UINT32 Size, PHX_RMD Descriptor);
DLL_EXPORT HX_RESULT HxFreeMemory(HX_RMD Descriptor);
DLL_EXPORT HX_RESULT HxMapDescriptor(HX_RMD Descriptor, HX_PROCESS AddressSpace, PVOID MapAddress, HX_MAP_OPERATION Operation);
DLL_EXPORT HX_RESULT HxDescribeMemory(UINT64 PhysicalAddress, UINT32 Size, PHX_RMD Descriptor);
DLL_EXPORT HX_RESULT HxTranslateAddress(PVOID VirtualAddress, HX_PROCESS AddressSpace, PUINT64 PhysicalAddress);

DLL_EXPORT HX_RESULT HxRegisterCallback(HX_OBJECT_TYPE ObjectType, HANDLE EventHandle, PVOID Memory, PHX_CALLBACK CallbackObject);
DLL_EXPORT HX_RESULT HxUnregisterCallback(HX_CALLBACK CallbackObject);

#endif // !HXPOSED