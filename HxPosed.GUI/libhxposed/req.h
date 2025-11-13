#ifndef REQ_HEADER
#define REQ_HEADER
#include "hxposed.h"

typedef struct hx_auth_request {
	guid_t guid;
	uint64_t permissions;
} hx_auth_request_t;

#endif