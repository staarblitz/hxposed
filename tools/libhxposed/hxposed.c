#include "hxposed.h"

// always required by compiler burh
void* memset(void* s, int c, size_t n) {
	unsigned char* p = s;
	while (n--) {
		*p++ = (unsigned char)c;
	}
	return s;
}

DLL_EXPORT UINT32 HxReadAsyncResponseLength(UINT64 Offset) {
	return *(PUINT32)(HX_ASYNC_BASE + Offset);
}

DLL_EXPORT PVOID HxReadAsyncResponseSlice(UINT64 Offset, PUINT32 Count) {
	if (Count)
		*Count = HxReadAsyncResponseLength(Offset);
	return (PVOID)(HX_ASYNC_BASE + Offset + 4);
}

DLL_EXPORT PVOID HxReadAsyncResponseType(UINT64 Offset) {
	UINT32 typeOffset = HxReadAsyncResponseLength(Offset);
	return (PVOID)(HX_ASYNC_BASE + typeOffset);
}

DLL_EXPORT BOOL HxGetStatus(PHXS_STATUS Response) {
	PHX_REQUEST_RESPONSE reqResp = { 0 };

	reqResp->Call.ServiceFunction = HxSvcGetState;

	if (HxpTrap(reqResp).ErrorCode != 0) {
		return FALSE;
	}

	Response->Status = reqResp->Arg1;
	Response->Version = reqResp->Arg2;

	return TRUE;
}

DLL_EXPORT HX_RESULT HxOpenObject(HX_SERVICE_FUNCTION Function, PVOID AddrOrId, PHX_OBJECT Object) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = Function,
		.OpenObjectRequest.AddressOrId = AddrOrId
	};

	HX_RESULT result = HxpTrap(&reqResp);

	*Object = (result.ErrorCode == 0) ? reqResp.OpenObjectResponse.Object.Object : *Object;
	return result;
}

DLL_EXPORT HX_RESULT HxCloseObject(HX_SERVICE_FUNCTION Function, HX_OBJECT Object) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = Function,
		.CloseObjectRequest.Address = Object
	};

	return HxpTrap(&reqResp);
}

HX_RESULT HxpGetObjectField(HX_SERVICE_FUNCTION Function, HX_OBJECT Object, UINT64 DataField, PUINT64 Field) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = Function,
		.GetProcessFieldRequest.Address = Object,
		.GetProcessFieldRequest.Data.Field = DataField,
	};

	HX_RESULT result = HxpTrap(&reqResp);

	*Field = reqResp.GetProcessFieldResponse.DirectoryTableBase;
	return result;
}

HX_RESULT HxpSetObjectField(HX_SERVICE_FUNCTION Function, HX_OBJECT Object, UINT64 DataField, PUINT64 Field) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = Function,
		.GetProcessFieldRequest.Address = Object,
		.GetProcessFieldRequest.Data.Field = DataField,
		.GetProcessFieldRequest.Data.DirectoryTableBase = *Field
	};

	return HxpTrap(&reqResp);
}

DLL_EXPORT HX_RESULT HxGetProcessNtPath(HX_PROCESS Process, PWCHAR* Name) {
	UINT64 Offset;
	HX_RESULT result = HxpGetObjectField(HxSvcGetProcessField, Process, HxProcessFieldNtPath, &Offset);
	*Name = HxReadAsyncResponseSlice(Offset, NULL);
	return result;
}

DLL_EXPORT HX_RESULT HxGetProcessThreads(HX_PROCESS Process, PUINT32* Threads, PUINT32 Count) {
	UINT64 Offset;
	HX_RESULT result = HxpGetObjectField(HxSvcGetProcessField, Process, HxProcessFieldThreads, &Offset);
	*Threads = HxReadAsyncResponseSlice(Offset, Count);
	return result;
}

DLL_EXPORT HX_RESULT HxGetTokenAccountName(HX_PROCESS Process, PWCHAR* Name) {
	UINT64 Offset;
	HX_RESULT result = HxpGetObjectField(HxSvcGetTokenField, Process, HxTokenFieldAccountName, &Offset);
	*Name = HxReadAsyncResponseSlice(Offset, NULL);
	return result;
}

DLL_EXPORT HX_RESULT HxReadMsr(UINT64 Msr, PUINT64 Value) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcMsrIo,
		.MsrIoRequest.Msr = Msr,
		.MsrIoRequest.Operation = HxMsrRead
	};

	HX_RESULT result = HxpTrap(&reqResp);
	*Value = reqResp.MsrIoResponse.Msr;
	return result;
}

DLL_EXPORT HX_RESULT HxWriteMsr(UINT64 Msr, UINT64 Value) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcMsrIo,
		.MsrIoRequest.Msr = Msr,
		.MsrIoRequest.Value = Value,
		.MsrIoRequest.Operation = HxMsrWrite
	};

	return HxpTrap(&reqResp);
}

DLL_EXPORT HX_RESULT HxExecPrivileged(HX_PRIVILEGED_INSTRUCTION Instruction, PUINT64 Result) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcExecutePrivilegedInstruction,
		.ExecutePrivilegedInstructionRequest.Instruction = Instruction
	};

	HX_RESULT result = HxpTrap(&reqResp);
	*Result = reqResp.ExecutePrivilegedInstructionResponse.Cr3;
	return result;
}

DLL_EXPORT HX_RESULT HxUpgradeHandle(UINT64 Handle, HX_PROCESS Process, UINT32 AccessMask) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcUpgradeHandle,
		 .UpgradeHandleRequest.Handle = Handle,
		 .UpgradeHandleRequest.AccessMask = AccessMask,
		 .UpgradeHandleRequest.Process = Process
	};

	return HxpTrap(&reqResp);
}

DLL_EXPORT HX_RESULT HxSwapHandleObject(UINT64 Handle, HX_PROCESS Process, HX_OBJECT NewObject) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcSwapHandleObject,
		.SwapHandleObjectRequest.Handle = Handle,
		.SwapHandleObjectRequest.NewObject = NewObject,
		.SwapHandleObjectRequest.Process = Process
	};

	return HxpTrap(&reqResp);
}

DLL_EXPORT HX_RESULT HxGetHandleObject(UINT64 Handle, HX_PROCESS Process, PHX_OBJECT Object, PUINT32 GrantedAccess) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcGetHandleObject,
		.GetHandleObjectRequest.Handle = Handle,
		.GetHandleObjectRequest.Process = Process
	};

	HX_RESULT result = HxpTrap(&reqResp);

	*Object = reqResp.GetHandleObjectResponse.Object;
	*GrantedAccess = reqResp.GetHandleObjectResponse.GrantedAccess;
	return result;
}

DLL_EXPORT HX_RESULT HxAllocateMemory(HX_MEMORY_POOL Pool, UINT32 Size, PHX_RMD Descriptor) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcAllocateMemory,
		.AllocateMemoryRequest.Pool = Pool,
		.AllocateMemoryRequest.Size = Size
	};

	HX_RESULT result = HxpTrap(&reqResp);
	*Descriptor = reqResp.AllocateMemoryResponse.RawMemoryDescriptor;
	return result;
}

DLL_EXPORT HX_RESULT HxFreeMemory(HX_RMD Descriptor) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcFreeMemory,
		.FreeMemoryRequest.Object = Descriptor
	};

	return HxpTrap(&reqResp);
}

DLL_EXPORT HX_RESULT HxMapDescriptor(HX_RMD Descriptor, HX_PROCESS AddressSpace, PVOID MapAddress, HX_MAP_OPERATION Operation) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcMapRawMemoryDescriptor,
		.Call.ExtendedArgsPresent = TRUE,
		.MapRawMemoryDescriptorRequest.AddressSpace = AddressSpace,
		.MapRawMemoryDescriptorRequest.MemoryDescriptor = Descriptor,
		.MapRawMemoryDescriptorRequest.MapAddress = MapAddress,
		.MapRawMemoryDescriptorRequest.Operation = Operation
	};

	return HxpTrap(&reqResp);
}

DLL_EXPORT HX_RESULT HxDescribeMemory(UINT64 PhysicalAddress, UINT32 Size, PHX_RMD Descriptor) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcDescribeMemory,
		.DescribeMemoryRequest.PhysicalAddress = PhysicalAddress,
		.DescribeMemoryRequest.Size = Size
	};

	HX_RESULT result = HxpTrap(&reqResp);
	*Descriptor = reqResp.DescribeMemoryResponse.RawMemoryDescriptor;

	return result;
}

DLL_EXPORT HX_RESULT HxTranslateAddress(PVOID VirtualAddress, HX_PROCESS AddressSpace, PUINT64 PhysicalAddress) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcTranslateAddress,
		.TranslateAddressRequest.AddressSpace = AddressSpace,
		.TranslateAddressRequest.VirtualAddress = VirtualAddress
	};

	HX_RESULT result = HxpTrap(&reqResp);
	*PhysicalAddress = reqResp.TranslateAddressResponse.PhysicalAddress;

	return result;
}

DLL_EXPORT HX_RESULT HxRegisterCallback(HX_OBJECT_TYPE ObjectType, HANDLE EventHandle, PHX_CALLBACK CallbackObject) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcRegisterNotifyEvent,
		.RegisterCallbackRequest.ObjectType = ObjectType,
		.RegisterCallbackRequest.EventHandle = EventHandle
	};

	HX_RESULT result = HxpTrap(&reqResp);
	*CallbackObject = reqResp.RegisterCallbackResponse.Object;
	return result;
}

DLL_EXPORT HX_RESULT HxUnregisterCallback(HX_CALLBACK CallbackObject) {
	HX_REQUEST_RESPONSE reqResp = {
		.Call.ServiceFunction = HxSvcRegisterNotifyEvent,
		.UnregisterCallbackRequest.Object = CallbackObject
	};

	return HxpTrap(&reqResp);
}

#define GENERATE_HX_FUNC_IMPL_GET(x, y, z) \
DLL_EXPORT HX_RESULT HxGet##x##y(HX_OBJECT x, z y) { \
    return HxpGetObjectField(HxSvcGet##x##Field, x, Hx##x##Field##y, y); \
}

#define GENERATE_HX_FUNC_IMPL_SET(x, y, z) \
DLL_EXPORT HX_RESULT HxSet##x##y(HX_OBJECT x, z y) { \
    return HxpSetObjectField(HxSvcSet##x##Field, x, Hx##x##Field##y, y); \
}

#define GENERATE_HX_FUNC_IMPL(x, y, z) \
	GENERATE_HX_FUNC_IMPL_GET(x,y,z) \
	GENERATE_HX_FUNC_IMPL_SET(x,y,z)


// trust the unions
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wincompatible-pointer-types"

GENERATE_HX_FUNC_IMPL(Process, Protection, PHX_PROCESS_PROTECTION)
GENERATE_HX_FUNC_IMPL(Process, Mitigation, PHX_PROCESS_MITIGATION_FLAGS)
GENERATE_HX_FUNC_IMPL(Process, Signers, PHX_PROCESS_SIGNERS)
GENERATE_HX_FUNC_IMPL(Process, Token, PUINT64)
GENERATE_HX_FUNC_IMPL(Process, DirectoryTableBase, PUINT64)

GENERATE_HX_FUNC_IMPL(Token, SourceName, PCHAR)
GENERATE_HX_FUNC_IMPL(Token, Type, PHX_TOKEN_TYPE)
GENERATE_HX_FUNC_IMPL(Token, IntegrityLevelIndex, PUINT32)
GENERATE_HX_FUNC_IMPL(Token, MandatoryPolicy, PUINT32)
GENERATE_HX_FUNC_IMPL(Token, ImpersonationLevel, PHX_TOKEN_IMPERSONATION_LEVEL)
GENERATE_HX_FUNC_IMPL(Token, PresentPrivileges, PHX_TOKEN_PRIVILEGES)
GENERATE_HX_FUNC_IMPL(Token, EnabledPrivileges, PHX_TOKEN_PRIVILEGES)
GENERATE_HX_FUNC_IMPL(Token, EnabledByDefaultPrivileges, PHX_TOKEN_PRIVILEGES)

GENERATE_HX_FUNC_IMPL(Thread, ActiveImpersonationInfo, PBOOL)
GENERATE_HX_FUNC_IMPL(Thread, AdjustedClientToken, PHX_TOKEN)

#pragma clang diagnostic pop

#undef GENERATE_HX_FUNC_IMPL
#undef GENERATE_HX_GET_FUNC_IMPL
#undef GENERATE_HX_SET_FUNC_IMPL