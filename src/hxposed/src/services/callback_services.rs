use crate::nt::callback::NtCallback;
use crate::nt::event::NtEvent;
use crate::nt::process::NtProcess;
use hxposed_core::hxposed::ObjectType;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::notify::{
    RegisterNotifyHandlerRequest, UnregisterNotifyHandlerRequest,
};
use hxposed_core::hxposed::responses::HxResponse;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::notify::CallbackInformation;
use crate::nt::mm::mdl::MemoryDescriptor;

pub fn register_callback_receiver(request: RegisterNotifyHandlerRequest) -> HxResponse {
    match request.target_object {
        ObjectType::Process(_) => {}
        ObjectType::Thread(_) => {}
        _ => return HxResponse::invalid_params(0),
    }

    let event = match NtEvent::from_handle(request.event_handle as _) {
        Ok(x) => x,
        Err(_) => return HxResponse::not_found_what(NotFoundReason::Event),
    };

    let process = NtProcess::current();
    let tracker = process.get_object_tracker_unchecked();
    let descriptor = match MemoryDescriptor::lock_pages(request.memory as _, size_of::<CallbackInformation>() as _) {
        Some(x) => x,
        None => return HxResponse::invalid_params(1)
    };
    tracker.add_callback(NtCallback::new(request.target_object, event, descriptor));

    EmptyResponse::default()
}

pub fn unregister_callback_receiver(request: UnregisterNotifyHandlerRequest) -> HxResponse {
    let process = NtProcess::current();
    let tracker = process.get_object_tracker_unchecked();

    match tracker.pop_open_callback(request.callback) {
        None => HxResponse::not_found_what(NotFoundReason::Callback),
        Some(x) => {
            drop(x);
            EmptyResponse::default()
        }
    }
}
