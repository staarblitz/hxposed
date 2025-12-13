#pragma once
#include "resp.h"
#include "req.h"
#include "hxposed.h"
#include <intrin.h>

HX_ERROR HxNotLoaded() {
	HX_ERROR err = { 0 };
	err.ErrorCode = HxErrNotLoaded;
	err.ErrorSource = HxSourceHx;
	return err;
}

HANDLE HxpCreateEvent() {
	return CreateEventA(NULL, TRUE, FALSE, NULL);
}

__declspec(dllexport) HX_ERROR HxGetStatus(PHXS_STATUS Response, HANDLE Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallGetStatus();

	if (HxpTrap(&reqResp) == -1) {
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
	UINT64* guid = &Auth->Guid;

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

	Response->Address = reqResp.Arg1;

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

__declspec(dllexport) HX_ERROR HxCloseThread(PHXR_CLOSE_THREAD Request, PHX_ASYNC_INFO Async) {
	HX_REQUEST_RESPONSE reqResp = { 0 };
	reqResp.Call = HxCallCloseThread();

	reqResp.Arg1 = Request->Id;

	if (HxpTrap(&reqResp, Async) == -1) {
		return HxNotLoaded();
	}

	return HxErrorFromResult(&reqResp.Result);
}