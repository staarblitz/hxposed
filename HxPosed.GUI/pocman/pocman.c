#include "hxposed.h"
#include <stdio.h>
#include <rpc.h>
#pragma comment(lib, "Rpcrt4.lib")

int main()
{
	UUID uuid;
	UuidFromStringW(L"{ca170835-4a59-4c6d-a04b-f5866f592c38}", &uuid);
	HXR_AUTH auth = {
		.Guid = uuid,
		.Permissions = MAXUINT64
	};

	HXS_AUTH response = { 0 };

	HX_ERROR error = HxAuthenticate(&auth, &response);
	if (HxIsError(&error)) {
		printf("Not authorized");
	}
}