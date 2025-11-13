#pragma once

#include <stdint.h>

typedef struct guid {
    uint32_t data1;
    uint16_t data2;
    uint16_t data3;
    uint64_t data4;
} guid_t;



// because C is garbage
typedef uint32_t error_source_t;
#define Nt 0
#define Hv 1
#define Hx 2

typedef uint32_t error_code_t;
#define Unknown 0
#define Ok 1
#define NotAllowed 2
#define NotLoaded 3

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

typedef uint64_t plugin_permissions_t;
#define NONE             0
#define PROCESS_EXECUTIVE1 << 0
#define PROCESS_MEMORY   1 << 1
#define PROCESS_PROTECTION1 << 2

#define RESERVED         1 << 3
#define RESERVED2        1 << 4
#define RESERVED3        1 << 5
#define RESERVED4        1 << 6
#define RESERVED5        1 << 7

#define MEMORY_VIRTUAL   1 << 8
#define MEMORY_PHYSICAL  1 << 9
#define MEMORY_ALLOCATION1 << 10
#define MEMORY_PROTECT   1 << 11
#define MEMORY_ISOLATE   1 << 12

#define RESERVED6        1 << 13
#define RESERVED7        1 << 14
#define RESERVED8        1 << 15
#define RESERVED9        1 << 16
#define RESERVED10       1 << 17

#define CPU_MSR_READ     1 << 18
#define CPU_MSR_WRITE    1 << 19
#define CPU_SEGMENTATION 1 << 20
#define CPU_CONTROL      1 << 21

#define RESERVED11       1 << 22
#define RESERVED12       1 << 23
#define RESERVED13       1 << 24
#define RESERVED14       1 << 25
#define RESERVED15       1 << 26

#define SECURITY_CREATE  1 << 27
#define SECURITY_MANAGE  1 << 28
#define SECURITY_DELETE  1 << 29