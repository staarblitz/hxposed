#ifndef RESP_HEADER
#define RESP_HEADER

#include <stdint.h>

typedef enum hypervisor_status {
    Unknown = 0,
    SystemVirtualized = 1,
    SystemDeVirtualized = 2,
} hypervisor_status_t;

typedef struct hx_status_response {
    hypervisor_status_t status;
    uint32_t version;
} hx_status_response_t;


#endif