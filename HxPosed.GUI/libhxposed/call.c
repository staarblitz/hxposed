#pragma once
#include "hxposed.h"

hypervisor_call_t call_get_status() {
	hypervisor_call_t call = { 0 };
	call.func = GetState;
	return call;
}

hypervisor_call_t call_auth() {
	hypervisor_call_t call = { 0 };
	call.func = Authorize;
	return call;
}