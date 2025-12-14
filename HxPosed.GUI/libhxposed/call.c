#pragma once
#include "hxposed.h"

HX_CALL HxCallGetStatus() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcGetState;
	return call;
}

HX_CALL HxCallAuth() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcAuthorize;
	return call;
}

HX_CALL HxCallOpenProcess() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcOpenProcess;
	return call;
}

HX_CALL HxCallOpenThread() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcOpenThread;
	return call;
}

HX_CALL HxCallOpenToken() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcOpenToken;
	return call;
}

HX_CALL HxCallCloseProcess() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcCloseProcess;
	return call;
}

HX_CALL HxCallCloseThread() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcCloseThread;
	return call;
}

HX_CALL HxCallCloseToken() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcCloseToken;
	return call;
}

HX_CALL HxCallGetProcessField() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcGetProcessField;
	call.ExtendedArgsPresent = TRUE;
	return call;
}

HX_CALL HxCallGetProcessThreads() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcGetProcessThreads;
	return call;
}

HX_CALL HxCallGetTokenField() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcGetTokenField;
	call.ExtendedArgsPresent = TRUE;
	return call;
}

HX_CALL HxCallGetThreadField() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcGetThreadField;
	call.ExtendedArgsPresent = TRUE;
	return call;
}

HX_CALL HxCallSetProcessField() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcSetProcessField;
	call.ExtendedArgsPresent = TRUE;
	return call;
}

HX_CALL HxCallSetThreadField() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcSetThreadField;
	call.ExtendedArgsPresent = TRUE;
	return call;
}

HX_CALL HxCallMapMemory() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcMapMemory;
	return call;
}

HX_CALL HxCallFreeMemory() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcFreeMemory;
	return call;
}

HX_CALL HxCallAllocateMemory() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcAllocateMemory;
	return call;
}

HX_CALL HxCallProcessVmOp() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcProcessVMOperation;
	return call;
}

HX_CALL HxCallProtectVm() {
	HX_CALL call = { 0 };
	call.ServiceFunction = HxSvcProcessVMOperation;
	return call;
}