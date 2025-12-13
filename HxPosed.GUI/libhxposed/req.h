#ifndef REQ_HEADER
#define REQ_HEADER
#include "hxposed.h"

typedef enum _HX_THREAD_FIELD {
	HxThreadFieldUnknown = 0,
	HxThreadFieldActiveImpersonationInfo = 1,
	HxThreadFieldAdjustedClientToken = 2,
} HX_THREAD_FIELD;

typedef enum _HX_TOKEN_FIELD {
	HxTokenFieldUnknown,
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
	HxProcFieldUnknown,
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
	HxPoolNonPaged
} HX_MEMORY_POOL;

typedef enum _HX_VM_OPERATION {
	HxVmRead = 0,
	HxVmWrite = 1
} HX_VM_OPERATION;

///////////////////////////////////////////////////////////////////////////////////////// BEGIN AUTH

typedef struct _HXR_AUTH {
	GUID Guid;
	UINT64 Permissions;
} HXR_AUTH, *PHXR_AUTH;

///////////////////////////////////////////////////////////////////////////////////////// END AUTH
///////////////////////////////////////////////////////////////////////////////////////// BEGIN MEMORY

typedef struct _HXR_RW_VM {
	UINT32 Id;
	PVOID Address;
	SIZE_T Count;
	PVOID Output;
	SIZE_T OutputSize;
	HX_VM_OPERATION Operation;
} HXR_RW_VM, *PHXR_RW_VM;

typedef struct _HXR_PROTECT_VM {
	UINT32 Id;
	PVOID Address;
	UINT32 Protection;
} HXR_PROTECT_VM, *PHXR_PROTECT_VM;

typedef struct _HXR_ALLOCATE_VM {
	UINT32 Id;
	UINT32 Reserved;
	HX_MEMORY_POOL Pool;
} HXR_ALLOCATE_VM, *PHXR_ALLOCATE_VM;

typedef struct _HXR_MAP_MEMORY {
	PVOID Mdl;
	PVOID MapAddress;
	HX_MAP_OPERATION Operation;
} HXR_MAP_MEMORY, *PHXR_MAP_MEMORY;

typedef struct _HXR_FREE_MEMORY {
	PVOID Mdl;
} HXR_FREE_MEMORY, *PHXR_FREE_MEMORY;

///////////////////////////////////////////////////////////////////////////////////////// END MEMORY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN PROCESS

typedef struct _HXR_OPEN_PROCESS {
	UINT32 Id;
	HX_OPEN_TYPE OpenType;
} HXR_OPEN_PROCESS, *PHXR_OPEN_PROCESS;

typedef struct _HXR_CLOSE_PROCESS {
	UINT32 Id;
} HXR_CLOSE_PROCESS, *PHXR_CLOSE_PROCESS;

typedef struct _HXR_KILL_PROCESS {
	UINT32 Id;
	UINT32 ExitCode;
} HXR_KILL_PROCESS, *PHXR_KILL_PROCESS;

typedef struct _HXR_GET_PROCESS_FIELD {
	UINT32 Id;
	HX_PROCESS_FIELD Field;
	PVOID Data;
	SIZE_T DataLen;
} HXR_GET_PROCESS_FIELD, *PHXR_GET_PROCESS_FIELD;

typedef struct _HXR_SET_PROCESS_FIELD {
	UINT32 Id;
	HX_PROCESS_FIELD Field;
	PVOID Data;
	SIZE_T DataLen;
} HXR_SET_PROCESS_FIELD, * PHXR_SET_PROCESS_FIELD;

typedef struct _HXR_GET_PROCESS_THREADS {
	UINT32 Id;
	PVOID Data;
	SIZE_T DataLen;
} HXR_GET_PROCESS_THREADS, * PHXR_GET_PROCESS_THREADS;

///////////////////////////////////////////////////////////////////////////////////////// END PROCESS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN SECURITY

typedef struct _HXR_OPEN_TOKEN {
	PVOID Address;
	HX_OPEN_TYPE OpenType;
} HXR_OPEN_TOKEN, * PHXR_OPEN_TOKEN;

typedef struct _HXR_CLOSE_TOKEN {
	PVOID Address;
} HXR_CLOSE_TOKEN, * PHXR_CLOSE_TOKEN;

typedef struct _HXR_GET_TOKEN_FIELD {
	UINT32 Id;
	HX_TOKEN_FIELD Field;
	PVOID Data;
	SIZE_T DataLen;
} HXR_GET_TOKEN_FIELD, * PHXR_GET_TOKEN_FIELD;

typedef struct _HXR_SET_TOKEN_FIELD {
	UINT32 Id;
	HX_TOKEN_FIELD Field;
	PVOID Data;
	SIZE_T DataLen;
} HXR_SET_TOKEN_FIELD, * PHXR_SET_TOKEN_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END SECURITY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN THREAD

typedef struct _HXR_OPEN_THREAD {
	UINT32 Id;
	HX_OPEN_TYPE OpenType;
} HXR_OPEN_THREAD, * PHXR_OPEN_THREAD;

typedef struct _HXR_CLOSE_THREAD {
	UINT32 Id;
} HXR_CLOSE_THREAD, * PHXR_CLOSE_THREAD;

typedef struct _HXR_GET_THREAD_FIELD {
	UINT32 Id;
	HX_THREAD_FIELD Field;
	PVOID Data;
	SIZE_T DataLen;
} HXR_GET_THREAD_FIELD, * PHXR_GET_THREAD_FIELD;

typedef struct _HXR_SET_THREAD_FIELD {
	UINT32 Id;
	HX_THREAD_FIELD Field;
	PVOID Data;
	SIZE_T DataLen;
} HXR_SET_THREAD_FIELD, * PHXR_SET_THREAD_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END THREAD
#endif