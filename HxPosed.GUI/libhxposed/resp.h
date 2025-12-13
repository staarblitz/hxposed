#ifndef RESP_HEADER
#define RESP_HEADER

#include "hxposed.h"
#include "req.h"
#include "fields.h"

typedef enum _HXS_HYPERVISOR_STATUS {
	HxStatusUnknown = 0,
	SystemVirtualized = 1,
	SystemDeVirtualized = 2,
} HXS_HYPERVISOR_STATUS;

typedef struct _HXS_OPEN_OBJECT_RESPONSE {
	PVOID Address;
} HXS_OPEN_OBJECT_RESPONSE, *PHXS_OPEN_OBJECT_RESPONSE;

///////////////////////////////////////////////////////////////////////////////////////// BEGIN STATUS

typedef struct _HXS_STATUS {
	HXS_HYPERVISOR_STATUS Status;
	UINT64 Version;
} HXS_STATUS, *PHXS_STATUS;


///////////////////////////////////////////////////////////////////////////////////////// END STATUS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN AUTH

typedef struct _HXS_AUTH {
	UINT64 Permissions;
} HXS_AUTH, *PHXS_AUTH;

///////////////////////////////////////////////////////////////////////////////////////// END AUTH
///////////////////////////////////////////////////////////////////////////////////////// BEGIN MEMORY

typedef struct _HXS_RW_VM {
	SIZE_T BytesProcesseed;
} HXS_RW_VM, *PHXS_RW_VM;

typedef struct _HXS_ALLOCATE_MEMORY {
	PVOID Address;
	UINT32 BytesAllocated;
} HXS_ALLOCATE_MEMORY, *PHXS_ALLOCATE_MEMORY;

typedef struct _HXS_MAP_MEMORY {
	PVOID MappedAddress;
} HXS_MAP_MEMORY, *PHXS_MAP_MEMORY;

///////////////////////////////////////////////////////////////////////////////////////// END MEMORY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN PROCESS

typedef struct _HXS_GET_PROCESS_FIELD {
	HX_PROCESS_FIELD Field;
	union _ProcessValues {
		struct _NtPath {
			UINT16 ByteLength;
		} NtPath;
		struct _Protection {
			HX_PROCESS_PROTECTION Protection;
		} Protection;
		struct _Signers {
			HX_PROCESS_SIGNERS Signers;
		} Signers;
		struct _Mitigation {
			HX_PROCESS_MITIGATION_FLAGS MitigationFlags;
		} Mitigation;
		struct _Token {
			UINT64 Token;
		} Token;
	} Values;
} HXS_GET_PROCESS_FIELD, *PHXS_GET_PROCESS_FIELD;

typedef struct _HXS_GET_PROCESS_THREADS {
	UINT32 NumberOfThreads;
} HXS_GET_PROCESS_THREADS, *PHXS_GET_PROCESS_THREADS;

///////////////////////////////////////////////////////////////////////////////////////// END PROCESS
///////////////////////////////////////////////////////////////////////////////////////// BEGIN SECURITY

typedef struct _HXS_GET_TOKEN_FIELD {
	HX_TOKEN_FIELD Field;
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
			HX_TOKEN_IMPERSONATION_LEVEL Level;
		} ImpersonationLevel;
		struct _Privileges {
			HX_TOKEN_PRIVILEGES Privileges;
		} Privileges;
	} Values;
} HXS_GET_TOKEN_FIELD, *PHXS_GET_TOKEN_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END SECURITY
///////////////////////////////////////////////////////////////////////////////////////// BEGIN THREAD

typedef struct _HXS_GET_THREAD_FIELD {
	HX_THREAD_FIELD Field;
	union _ThreadValues {
		struct _ActiveImpersonationInfo {
			BOOL Status;
		} ActiveImpersonationInfo;
		struct _AdjustClientToken {
			PVOID Token;
		} AdjustedClientToken;
	} Values;
} HXS_GET_THREAD_FIELD, *PHXS_GET_THREAD_FIELD;

///////////////////////////////////////////////////////////////////////////////////////// END THREAD

#endif