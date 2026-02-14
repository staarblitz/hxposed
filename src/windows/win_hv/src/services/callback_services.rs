use crate::nt::callback::NtCallback;
use crate::nt::event::NtEvent;
use crate::nt::process::NtProcess;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::notify::{
    RegisterNotifyHandlerRequest, UnregisterNotifyHandlerRequest,
};
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::hxposed::responses::empty::EmptyResponse;

pub fn register_callback_receiver(request: RegisterNotifyHandlerRequest) -> HypervisorResponse {
    let event = match NtEvent::from_handle(request.event_handle as _) {
        Ok(x) => x,
        Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Event),
    };

    let process = NtProcess::current();
    let tracker = process.get_object_tracker_unchecked();
    tracker.add_callback(NtCallback::new(request.target_object, event));

    EmptyResponse::with_service(ServiceFunction::RegisterNotifyEvent)
}

pub fn unregister_callback_receiver(request: UnregisterNotifyHandlerRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let tracker = process.get_object_tracker_unchecked();

    match tracker.pop_open_callback(request.callback) {
        None => HypervisorResponse::not_found_what(NotFoundReason::Callback),
        Some(_x) => {
            // drop the obj

            EmptyResponse::with_service(ServiceFunction::UnregisterNotifyEvent)
        }
    }
}
