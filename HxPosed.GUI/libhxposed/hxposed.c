#pragma once,
#include "resp.h"
#include "hxposed.h"


__declspec(dllexport) hypervisor_error_t get_hx_state(hx_status_response_t* resp) {
	hypervisor_error_t err = { 0 };
	hypervisor_req_resp_t req_resp = { 0 };
	req_resp.call = call_get_status();

	trap(&req_resp);

	resp->status = req_resp.arg1;
	resp->version = req_resp.arg2;

	return err_from_result(&req_resp.result);
}