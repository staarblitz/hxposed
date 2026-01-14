use crate::nt::callback::NtCallback;
use crate::objects::ObjectTracker;
use crate::services::commands::callback::AwaitNotificationRequestAsyncCommand;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::notify::{
    AwaitNotificationRequest, RegisterNotifyHandlerRequest, UnregisterNotifyHandlerRequest,
};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::notify::RegisterNotifyHandlerResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::ObjectType;

pub(crate) fn register_callback(
    request: RegisterNotifyHandlerRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    match request.target_object {
        ObjectType::Process(_) | ObjectType::Thread(_) => {}
        _ => {
            return HypervisorResponse::invalid_params(ServiceParameter::Arg1);
        }
    };

    let callback = NtCallback::new(request.target_object);
    let addr = callback.callback;

    ObjectTracker::add_callback(callback);

    RegisterNotifyHandlerResponse { callback: addr }.into_raw()
}

pub(crate) fn await_notification(
    request: AwaitNotificationRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    match ObjectTracker::get_callback(request.callback) {
        None => HypervisorResponse::not_found_what(NotFoundReason::Callback),
        Some(x) => {
            x.queue_callback_waiter(AwaitNotificationRequestAsyncCommand {
                async_info,
                command: request,
            });
            EmptyResponse::with_service(ServiceFunction::AwaitNotifyEvent)
        }
    }
}

pub(crate) fn unregister_callback(
    request: UnregisterNotifyHandlerRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    match ObjectTracker::get_callback(request.callback) {
        None => HypervisorResponse::not_found_what(NotFoundReason::Callback),
        Some(x) => {
            x.take(); // forget
            EmptyResponse::with_service(ServiceFunction::UnregisterNotifyEvent)
        }
    }
}
