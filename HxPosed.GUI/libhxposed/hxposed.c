#pragma once
#include "hxposed.h"

HX_ERROR HxNotLoaded() {
	HX_ERROR err = { 0 };
	err.ErrorCode = HxErrNotLoaded;
	err.ErrorSource = HxSourceHx;
	return err;
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

__declspec(dllexport) HX_ERROR HxAuthenticate(PHXR_AUTH Auth, PHXS_AUTH Response) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallAuth();

	// 200000iq
	PUINT64 guid = &Auth->Guid;

	reqResp.Arg1 = guid[0];
	reqResp.Arg2 = guid[1];
	reqResp.Arg3 = Auth->Permissions;

	if (HxpTrap(&reqResp, NULL) == -1) {
		return HxNotLoaded();
	}

	Response->Permissions = reqResp.Arg1;

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxOpenProcess(PHXR_OPEN_PROCESS Request, PHXS_OPEN_OBJECT_RESPONSE Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallOpenProcess();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->OpenType;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	if (Async != NULL) {
		Response->Address = reqResp.Arg1;
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxGetProcessField(PHXR_GET_PROCESS_FIELD Request, PHXS_GET_PROCESS_FIELD Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallOpenProcess();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->Field;

	reqResp.ExtendedArg1.Low = Request->Data;
	reqResp.ExtendedArg2.Low = Request->DataLen;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	if (Async != NULL) {
		Response->Field = reqResp.Arg1;
		*(PUINT64)(&Response->ProcessValues) = reqResp.Arg2;
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxSetProcessField(PHXR_SET_PROCESS_FIELD Request, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallOpenProcess();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->Field;

	reqResp.ExtendedArg1.Low = Request->Data;
	reqResp.ExtendedArg2.Low = Request->DataLen;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxGetProcessThreads(PHXR_GET_PROCESS_THREADS Request, PHXS_GET_PROCESS_THREADS Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallGetProcessThreads();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->Data;
	reqResp.Arg3 = Request->DataLen;
	
	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	if (Async != NULL) {
		Response->NumberOfThreads = reqResp.Arg1;
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxCloseProcess(PHXR_CLOSE_PROCESS Request, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallCloseProcess();

	reqResp.Arg1 = Request->Id;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxOpenThread(PHXR_OPEN_THREAD Request, PHXS_OPEN_OBJECT_RESPONSE Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallOpenThread();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->OpenType;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxGetThreadField(PHXR_GET_THREAD_FIELD Request, PHXS_GET_THREAD_FIELD Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallOpenThread();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->Field;

	reqResp.ExtendedArg1.Low = Request->Data;
	reqResp.ExtendedArg2.Low = Request->DataLen;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	if (Async != NULL) {
		Response->Field = reqResp.Arg1;
		*(PUINT64)(&Response->ThreadValues) = reqResp.Arg2;
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxSetThreadField(PHXR_SET_THREAD_FIELD Request, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallOpenThread();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->Field;

	reqResp.ExtendedArg1.Low = Request->Data;
	reqResp.ExtendedArg2.Low = Request->DataLen;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxCloseThread(PHXR_CLOSE_THREAD Request, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallCloseThread();

	reqResp.Arg1 = Request->Id;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxOpenToken(PHXR_OPEN_TOKEN Request, PHXS_OPEN_OBJECT_RESPONSE Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallOpenToken();

	reqResp.Arg1 = Request->Address;
	reqResp.Arg2 = Request->OpenType;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxGetTokenField(PHXR_GET_TOKEN_FIELD Request, PHXS_GET_TOKEN_FIELD Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallOpenToken();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->Field;

	reqResp.ExtendedArg1.Low = Request->Data;
	reqResp.ExtendedArg2.Low = Request->DataLen;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	if (Async != NULL) {
		Response->Field = reqResp.Arg1;
		*(PUINT64)(&Response->TokenValues) = reqResp.Arg2;
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxSetTokenField(PHXR_SET_TOKEN_FIELD Request, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallOpenToken();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->Field;

	reqResp.ExtendedArg1.Low = Request->Data;
	reqResp.ExtendedArg2.Low = Request->DataLen;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}


__declspec(dllexport) HX_ERROR HxCloseToken(PHXR_CLOSE_TOKEN Request, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallCloseToken();

	reqResp.Arg1 = Request->Address;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxAllocateMemory(PHXR_ALLOCATE_MEMORY Request, PHXS_ALLOCATE_MEMORY Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallAllocateMemory();

	reqResp.Arg1 = Request->Size;
	reqResp.Arg2 = Request->Reserved;
	reqResp.Arg3 = Request->Pool;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	if (Async != NULL) {
		Response->Address = reqResp.Arg1;
		Response->BytesAllocated = reqResp.Arg2;
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxFreeMemory(PHXR_FREE_MEMORY Request, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallFreeMemory();

	reqResp.Arg1 = Request->Mdl;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxMapMemory(PHXR_MAP_MEMORY Request, PHXS_MAP_MEMORY Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallMapMemory();

	reqResp.Arg1 = Request->Mdl;
	reqResp.Arg2 = Request->MapAddress;
	reqResp.Arg3 = Request->Operation;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	if (Async != NULL) {
		Response->MappedAddress = reqResp.Arg1;
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxVmOperation(PHXR_RW_VM Request, PHXS_RW_VM Response, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallProcessVmOp();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->Address;
	reqResp.Arg3 = Request->Count;

	reqResp.ExtendedArg1.Low = Request->Output;
	reqResp.ExtendedArg1.Low = Request->OutputSize;
	reqResp.ExtendedArg1.Low = Request->Operation;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	if (Async != NULL) {
		Response->BytesProcesseed = reqResp.Arg1;
	}

	return HxErrorFromResult(&reqResp.Result);
}

__declspec(dllexport) HX_ERROR HxProtectVm(PHXR_PROTECT_VM Request, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallProcessVmOp();

	reqResp.Arg1 = Request->Id;
	reqResp.Arg2 = Request->Address;
	reqResp.Arg3 = Request->Protection;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}