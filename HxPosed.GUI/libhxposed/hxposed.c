#include "hxposed.h"

HX_ERROR HxNotLoaded() {
	HX_ERROR err;
	err.ErrorCode = HxErrNotLoaded;
	err.ErrorSource = HxSourceHx;
	return err;
}

HX_ERROR HxOk() {
	HX_ERROR err;
	err.ErrorCode = HxErrOk;
	err.ErrorSource = HxSourceHx;
	return err;
}

__declspec(dllexport) HX_ERROR HxpResponseFromRaw(PHX_REQUEST_RESPONSE RequestResponse, PVOID Response) {
	HX_ERROR error = HxErrorFromResult(&RequestResponse->Result);

	if (HxIsError(&error)) {
		HeapFree(GetProcessHeap(), 0, RequestResponse);
		return error;
	}

	switch (RequestResponse->Result.ServiceFunction) {
	case HxSvcGetState: {
		PHXS_STATUS result = Response;
		result->Status = RequestResponse->Arg1;
		result->Version = RequestResponse->Arg2;
		break;
	}

	case HxSvcOpenProcess: {
		PHXS_OPEN_OBJECT_RESPONSE result = Response;
		result->Address = RequestResponse->Arg1;
		break;
	}
	case HxSvcOpenThread: {
		PHXS_OPEN_OBJECT_RESPONSE result = Response;
		result->Address = RequestResponse->Arg1;
		break;
	}
	case HxSvcOpenToken: {
		PHXS_OPEN_OBJECT_RESPONSE result = Response;
		result->Address = RequestResponse->Arg1;
		break;
	}
	case HxSvcGetSetPageAttribute: {
		PHXS_GET_SET_PAGE_ATTRIBUTE result = Response;
		result->TypeBits = RequestResponse->Arg1;
		break;
	}
	case HxSvcAllocateMemory: {
		PHXS_ALLOCATE_MEMORY result = Response;
		result->SystemVA = RequestResponse->Arg1;
		break;
	}
	case HxSvcRegisterNotifyEvent: {
		PHXS_REGISTER_CALLBACK result = Response;
		result->Object = RequestResponse->Arg1;
		break;
	}
	case HxSvcGetProcessField: {
		PHXS_GET_PROCESS_FIELD result = Response;
		result->Field = RequestResponse->Arg1;
		result->ThreadsOffset = RequestResponse->Arg2;
		break;
	}
	case HxSvcGetTokenField: {
		PHXS_GET_TOKEN_FIELD result = Response;
		result->Field = RequestResponse->Arg1;
		result->NameOffset = RequestResponse->Arg2;
		break;
	}
	case HxSvcGetThreadField: {
		PHXS_GET_THREAD_FIELD result = Response;
		result->Field = RequestResponse->Arg1;
		result->Token = RequestResponse->Arg2;
		break;
	}
	}

	HeapFree(GetProcessHeap(), 0, RequestResponse);

	return HxOk();
}

__declspec(dllexport) PHX_REQUEST_RESPONSE HxpRawFromRequest(HX_SERVICE_FUNCTION Function, PVOID Request) {
	PHX_REQUEST_RESPONSE reqResp = HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, sizeof(HX_REQUEST_RESPONSE));
	reqResp->Call.ServiceFunction = Function;

	switch (Function) {
	case HxSvcAllocateMemory: {
		PHXR_ALLOCATE_MEMORY req = (PHXR_ALLOCATE_MEMORY)Request;
		reqResp->Arg1 = req->Size;
		reqResp->Arg2 = req->Pool;
		break;
	}
	case HxSvcFreeMemory: {
		PHXR_FREE_MEMORY req = (PHXR_FREE_MEMORY)Request;
		reqResp->Arg1 = req->Object;
		break;
	}
	case HxSvcMapVaToPa: {
		PHXR_MAP_VA_TO_PA req = (PHXR_MAP_VA_TO_PA)Request;
		reqResp->Call.ExtendedArgsPresent = TRUE;
		reqResp->Arg1 = req->MemoryDescriptor;
		reqResp->Arg2 = req->AddressSpace;
		reqResp->Arg3 = req->MapAddress;
		reqResp->ExtendedArg1 = req->Operation;
		break;
	}
	case HxSvcGetSetPageAttribute: {
		PHXR_GET_SET_PAGE_ATTRIBUTE req = (PHXR_GET_SET_PAGE_ATTRIBUTE)Request;
		reqResp->Call.ExtendedArgsPresent = TRUE;
		reqResp->Arg1 = req->AddressSpace;
		reqResp->Arg2 = req->Operation;
		reqResp->Arg3 = req->TypeBits;
		reqResp->ExtendedArg1 = req->PagingType.ObjectType;
		reqResp->ExtendedArg2 = req->PagingType.Object.Address;
		break;
	}
	case HxSvcRegisterNotifyEvent: {
		PHXR_REGISTER_CALLBACK req = (PHXR_REGISTER_CALLBACK)Request;
		reqResp->Arg1 = req->ObjectType;
		reqResp->Arg2 = 0;
		reqResp->Arg3 = req->EventHandle;
		break;
	}
	case HxSvcUnregisterNotifyEvent: {
		PHXR_UNREGISTER_CALLBACK req = (PHXR_UNREGISTER_CALLBACK)Request;
		reqResp->Arg1 = req->Object;
		break;
	}
	case HxSvcOpenProcess: {
		PHXR_OPEN_PROCESS req = (PHXR_OPEN_PROCESS)Request;
		reqResp->Arg1 = req->Id;
		reqResp->Arg2 = req->OpenType;
		break;
	}
	case HxSvcCloseProcess: {
		PHXR_CLOSE_PROCESS req = (PHXR_CLOSE_PROCESS)Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->OpenType;
		break;
	}
	case HxSvcGetProcessField: {
		PHXR_GET_PROCESS_FIELD req = (PHXR_GET_PROCESS_FIELD)Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Data.Field;
		reqResp->Arg3 = req->Data.NtPathOffset; // use the largest member so it includes all variants
		break;
	}
	case HxSvcSetProcessField: {
		PHXR_SET_PROCESS_FIELD req = (PHXR_SET_PROCESS_FIELD)Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Data.Field;
		reqResp->Arg3 = req->Data.NtPathOffset;
		break;
	}
	case HxSvcOpenToken: {
		PHXR_OPEN_TOKEN req = (PHXR_OPEN_TOKEN)Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->OpenType;
		break;
	}
	case HxSvcCloseToken: {
		PHXR_CLOSE_TOKEN req = (PHXR_CLOSE_TOKEN)Request;
		reqResp->Arg1 = req->Address;
		break;
	}
	case HxSvcGetTokenField: {
		PHXR_GET_TOKEN_FIELD req = (PHXR_GET_TOKEN_FIELD)Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Data.Field;
		reqResp->Arg3 = req->Data.NameOffset;
		break;
	}
	case HxSvcSetTokenField: {
		PHXR_SET_TOKEN_FIELD req = (PHXR_SET_TOKEN_FIELD)Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Data.Field;
		reqResp->Arg3 = req->Data.NameOffset;
		break;
	}
	case HxSvcOpenThread: {
		PHXR_OPEN_THREAD req = (PHXR_OPEN_THREAD)Request;
		reqResp->Arg1 = req->Id;
		reqResp->Arg2 = req->OpenType;
		break;
	}
	case HxSvcCloseThread: {
		PHXR_CLOSE_THREAD req = (PHXR_CLOSE_THREAD)Request;
		reqResp->Arg1 = req->Address;
		break;
	}
	case HxSvcGetThreadField: {
		PHXR_GET_THREAD_FIELD req = (PHXR_GET_THREAD_FIELD)Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Data.Field;
		reqResp->Arg3 = req->Data.Token;
		break;
	}
	case HxSvcSetThreadField: {
		PHXR_GET_THREAD_FIELD req = (PHXR_GET_THREAD_FIELD)Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Data.Field;
		reqResp->Arg3 = req->Data.Token;
		break;
	}
	}

	return reqResp;
}

__declspec(dllexport) UINT32 HxReadAsyncResponseLength(UINT64 Offset) {
	return *(PUINT32)(HX_ASYNC_BASE + Offset);
}

__declspec(dllexport) PVOID HxReadAsyncResponseSlice(UINT64 Offset, PUINT32 Length) {
	*Length = HxReadAsyncResponseLength(Offset);
	return (PVOID)(HX_ASYNC_BASE + Offset + 4);
}

__declspec(dllexport) PVOID HxReadAsyncResponseType(UINT64 Offset) {
	UINT32 typeOffset = HxReadAsyncResponseLength(Offset);
	return (PVOID)(HX_ASYNC_BASE + typeOffset);
}

__declspec(dllexport) HX_ERROR HxGetStatus(PHXS_STATUS Response) {
	HX_REQUEST_RESPONSE reqResp;

	if (HxpTrap(&reqResp) == -1) {
		return HxNotLoaded();
	}

	Response->Status = reqResp.Arg1;
	Response->Version = reqResp.Arg2;

	return HxErrorFromResult(&reqResp.Result);
}