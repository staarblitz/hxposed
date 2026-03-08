#include "hxposed.h"

// not really async

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

__declspec(dllexport) BOOL HxGetStatus(PHXS_STATUS Response) {
	// allocating from heap since i dont have memset to initialize the local variable to zero
	PHX_REQUEST_RESPONSE reqResp = (PHX_REQUEST_RESPONSE)HeapAlloc(GetProcessHeap(), HEAP_ZERO_MEMORY, sizeof(HX_REQUEST_RESPONSE));

	reqResp->Call.ServiceFunction = HxSvcGetState;

	if (HxpTrap(reqResp) == -1) {
		HeapFree(GetProcessHeap(), NULL, reqResp);
		return FALSE;
	}

	Response->Status = reqResp->Arg1;
	Response->Version = reqResp->Arg2;

	return TRUE;
}