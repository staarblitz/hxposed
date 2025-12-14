#pragma once
#include "hxposed.h"

HX_ERROR HxNotLoaded() {
	HX_ERROR err = { 0 };
	err.ErrorCode = HxErrNotLoaded;
	err.ErrorSource = HxSourceHx;
	return err;
}

HX_ERROR HxOk() {
	HX_ERROR err = { 0 };
	err.ErrorCode = HxErrOk;
	err.ErrorSource = HxSourceHx;
	return err;
}

__declspec(dllexport) HX_ERROR HxpResponseFromAsync(PHX_ASYNC_INFO Async, PVOID Result) {
	HX_ERROR error = HxErrorFromResult(&Async->Result);

	if (HxIsError(&error)) {
		return error;
	}

	switch (Async->Result.ServiceFunction) {
	case HxSvcGetProcessField: {
		PHXS_GET_PROCESS_FIELD result = Result;
		result->Field = Async->Arg1;
		*(PUINT64)(&result->ProcessValues) = Async->Arg2;
		break;
	case HxSvcGetProcessThreads: {
		PHXS_GET_PROCESS_THREADS result = Result;
		result->NumberOfThreads = Async->Arg1;
		break;
	}
	case HxSvcGetTokenField: {
		PHXS_GET_PROCESS_FIELD result = Result;
		result->Field = Async->Arg1;
		*(PUINT64)(&result->ProcessValues) = Async->Arg2;
		break;
	}
	case HxSvcGetThreadField: {
		PHXS_GET_PROCESS_FIELD result = Result;
		result->Field = Async->Arg1;
		*(PUINT64)(&result->ProcessValues) = Async->Arg2;
		break;
	}
	case HxSvcMapMemory: {
		PHXS_MAP_MEMORY result = Result;
		result->MappedAddress = Async->Arg1;
		break;
	}
	case HxSvcOpenProcess: {
		PHXS_OPEN_OBJECT_RESPONSE result = Result;
		result->Address = Async->Arg1;
		break;
	}
	case HxSvcOpenThread: {
		PHXS_OPEN_OBJECT_RESPONSE result = Result;
		result->Address = Async->Arg1;
		break;
	}
	case HxSvcOpenToken: {
		PHXS_OPEN_OBJECT_RESPONSE result = Result;
		result->Address = Async->Arg1;
		break;
	}
	}
	}

	return HxOk();
}

__declspec(dllexport) HX_ERROR HxpResponseFromRaw(PHX_REQUEST_RESPONSE RequestResponse, PVOID Response) {
	HX_ERROR error = HxErrorFromResult(&RequestResponse->Result);

	if (RequestResponse->Call.IsAsync) {
		HeapFree(GetProcessHeap(), 0, RequestResponse);
		return error;
	}

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
	case HxSvcAuthorize: {
		PHXS_AUTH result = Response;
		result->Permissions = RequestResponse->Arg1;
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
	}

	HeapFree(GetProcessHeap(), 0, RequestResponse);

	return HxOk();
}

__declspec(dllexport) PHX_REQUEST_RESPONSE HxpRawFromRequest(HX_SERVICE_FUNCTION Function, PVOID Request) {
	PHX_REQUEST_RESPONSE reqResp = HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, sizeof(HX_REQUEST_RESPONSE));
	reqResp->Call.ServiceFunction = Function;

	switch (Function) {
	case HxSvcGetState: {
		break;
	}
	case HxSvcAuthorize: {
		PHXR_AUTH req = Request;
		reqResp->Arg1 = req->Guid.Low;
		reqResp->Arg2 = req->Guid.High;
		reqResp->Arg3 = req->Permissions;
		break;
	}
	case HxSvcOpenProcess: {
		PHXR_OPEN_PROCESS req = Request;
		reqResp->Arg1 = req->Id;
		reqResp->Arg2 = req->OpenType;
		break;
	}
	case HxSvcCloseProcess: {
		PHXR_CLOSE_PROCESS req = Request;
		reqResp->Arg1 = req->Address;
		break;
	}
	case HxSvcGetProcessField: {
		PHXR_GET_PROCESS_FIELD req = Request;
		reqResp->Call.ExtendedArgsPresent = TRUE;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Field;

		reqResp->ExtendedArg1.Low = req->Data;
		reqResp->ExtendedArg2.Low = req->DataLen;
		break;
	}
	case HxSvcSetProcessField: {
		PHXR_SET_PROCESS_FIELD req = Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Field;

		reqResp->ExtendedArg1.Low = req->Data;
		reqResp->ExtendedArg2.Low = req->DataLen;
		break;
	}
	case HxSvcGetProcessThreads: {
		PHXR_GET_PROCESS_THREADS req = Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Data;
		reqResp->Arg3 = req->DataLen;
		break;
	}
	case HxSvcOpenThread: {
		PHXR_OPEN_THREAD req = Request;
		reqResp->Arg1 = req->Id;
		reqResp->Arg2 = req->OpenType;
		break;
	}
	case HxSvcCloseThread: {
		PHXR_CLOSE_THREAD req = Request;
		reqResp->Arg1 = req->Address;
		break;
	}
	case HxSvcGetThreadField: {
		PHXR_GET_THREAD_FIELD req = Request;
		reqResp->Call.ExtendedArgsPresent = TRUE;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Field;

		reqResp->ExtendedArg1.Low = req->Data;
		reqResp->ExtendedArg2.Low = req->DataLen;
		break;
	}
	case HxSvcSetThreadField: {
		PHXR_SET_THREAD_FIELD req = Request;
		reqResp->Call.ExtendedArgsPresent = TRUE;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Field;

		reqResp->ExtendedArg1.Low = req->Data;
		reqResp->ExtendedArg2.Low = req->DataLen;
		break;
	}
	case HxSvcOpenToken: {
		PHXR_OPEN_TOKEN req = Request;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->OpenType;
		break;
	}
	case HxSvcCloseToken: {
		PHXR_CLOSE_TOKEN req = Request;
		reqResp->Arg1 = req->Address;
		break;
	}
	case HxSvcGetTokenField: {
		PHXR_GET_TOKEN_FIELD req = Request;
		reqResp->Call.ExtendedArgsPresent = TRUE;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Field;

		reqResp->ExtendedArg1.Low = req->Data;
		reqResp->ExtendedArg2.Low = req->DataLen;
		break;
	}
	case HxSvcSetTokenField: {
		PHXR_SET_TOKEN_FIELD req = Request;
		reqResp->Call.ExtendedArgsPresent = TRUE;
		reqResp->Arg1 = req->Address;
		reqResp->Arg2 = req->Field;

		reqResp->ExtendedArg1.Low = req->Data;
		reqResp->ExtendedArg2.Low = req->DataLen;
		break;
	}
	case HxSvcAllocateMemory: {
		PHXR_ALLOCATE_MEMORY req = Request;
		reqResp->Arg1 = req->Size;
		reqResp->Arg2 = req->Reserved;
		reqResp->Arg3 = req->Pool;
		break;
	}
	case HxSvcMapMemory: {
		PHXR_MAP_MEMORY req = Request;
		reqResp->Arg1 = req->Mdl;
		reqResp->Arg2 = req->MapAddress;
		reqResp->Arg3 = req->Operation;
		break;
	}
	case HxSvcFreeMemory: {
		PHXR_FREE_MEMORY req = Request;
		reqResp->Arg1 = req->Mdl;
		break;
	}
	}

	return reqResp;
}

__declspec(dllexport) HANDLE HxpCreateEventHandle() {
	return CreateEventA(NULL, TRUE, FALSE, NULL);
}

__declspec(dllexport) HX_ERROR HxGetStatus(PHXS_STATUS Response) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallGetStatus();

	if (HxpTrap(&reqResp, NULL) == -1) {
		return HxNotLoaded();
	}

	Response->Status = reqResp.Arg1;
	Response->Version = reqResp.Arg2;

	return HxErrorFromResult(&reqResp.Result);
}