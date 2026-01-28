#include "hxposed.h"

BOOL HxIsError(PHX_ERROR Error) {
	return !(Error->ErrorCode == HxErrOk && Error->ErrorSource == HxSourceHx);
}

HX_ERROR HxErrorFromResult(PHX_RESULT Result) {
	HX_ERROR error;
	error.ErrorCode = Result->ErrorCode;
	error.ErrorSource = Result->ErrorSource;
	return error;
}