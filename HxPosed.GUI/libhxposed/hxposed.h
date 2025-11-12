#pragma once

#include <stdint.h>


typedef enum error_source {
    Nt = 0,
    Hv = 1,
    Hx = 2,
} error_source_t;


typedef enum error_code {
    Ok = 1,
    NotAllowed = 2,
    NotLoaded = 3
} error_code_t;

typedef enum service_function {
    Authorize = 1,
    GetState = 2,
    OpenProcess = 3
} service_function_t;

typedef struct hypervisor_error {
    error_source_t source;
    error_code_t code;
} hypervisor_error_t;

static_assert(sizeof(hypervisor_error_t) == sizeof(uint64_t), "Invalid size");


#pragma pack(push,1)
typedef struct hypervisor_result {
    service_function_t func : 16;
    error_source_t source : 2;
    error_code_t code : 3;
    uint32_t cookie : 11;
} hypervisor_result_t;

typedef struct hypervisor_call {
    service_function_t func : 16;
    uint32_t is_fast : 1;
    uint32_t ignore_result : 1;
    uint32_t buffer_by_user : 1;
    uint32_t yield_execution : 1;
    uint32_t is_async : 1;
    uint32_t async_cookie : 11;
} hypervisor_call_t;

typedef struct hypervisor_req_resp {
    hypervisor_call_t call;
    hypervisor_result_t result;

    // args
    uint64_t arg1;
    uint64_t arg2;
    uint64_t arg3;
} hypervisor_req_resp_t;
#pragma pack(pop)

static_assert(sizeof(hypervisor_result_t) == sizeof(uint32_t), "Invalid size");
static_assert(sizeof(hypervisor_call_t) == sizeof(uint32_t), "Invalid size");

extern int __fastcall trap(hypervisor_req_resp_t* req_resp);
int err_is_error(hypervisor_error_t* err);
hypervisor_error_t err_from_result(hypervisor_result_t* result);

hypervisor_call_t call_get_status();
hypervisor_call_t call_auth();