#pragma once,
#include "resp.h"
#include "hxposed.h"
#include <intrin.h>


__declspec(dllexport) hypervisor_error_t get_hx_state(hx_status_response_t* resp) {
	hypervisor_req_resp_t req_resp = { 0 };
	req_resp.call = call_get_status();

	if (trap(&req_resp) == -1) {
		hypervisor_error_t err = { 0 };
		err.code = NotLoaded;
		err.source = Hx;
		return err;
	}

	resp->status = req_resp.arg1;
	resp->version = req_resp.arg2;

	return err_from_result(&req_resp.result);
}