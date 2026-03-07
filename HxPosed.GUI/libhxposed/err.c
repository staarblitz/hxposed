#include "hxposed.h"

BOOL HxIsError(PHX_RESULT Error) {
	return Error->ErrorCode != 0;
}