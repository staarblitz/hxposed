#pragma once
#include "hxposed.h"

int err_is_error(hypervisor_error_t* err) {
	return !(err->code == Ok || err->source == Hx);
}

hypervisor_error_t err_from_result(hypervisor_result_t* result) {
	hypervisor_error_t error = { 0 };
	error.code = result->code;
	error.source = result->source;
	return error;
}