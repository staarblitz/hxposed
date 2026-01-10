use egui::{Color32, ViewportBuilder};
use hxposed_core::error::HypervisorError;
use hxposed_core::hxposed::requests::Vmcall;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::status::StatusRequest;
use hxposed_core::hxposed::responses::auth::AuthorizationResponse;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::memory::HxMemory;
use hxposed_core::services::memory_map::{HxMemoryDescriptor, HxMemoryGuard};
use hxposed_core::services::process::HxProcess;
use hxposed_core::services::types::memory_fields::MemoryPool;
use hxposed_core::services::types::process_fields::{
    ProcessProtection, ProtectionSigner, ProtectionType,
};
use std::borrow::ToOwned;
use std::fmt;
use std::fmt::{Debug, format};
use std::ops::DerefMut;
use std::str::FromStr;
use uuid::{Error, Uuid};

fn main() {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([500.0, 400.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "HxTest",
        options,
        Box::new(|_| Ok(Box::<HxTestApp>::default())),
    );
}

#[derive(Default)]
struct HxTestApp {
    state: AppState,
    status_label_text: String,
    status_label_color: Color32,
}

impl HxTestApp {
    fn set_error(&mut self, text: String) {
        self.status_label_text = text;
        self.status_label_color = Color32::from_rgb(255, 0, 0);
    }
    fn set_ok(&mut self, text: String) {
        self.status_label_text = text;
        self.status_label_color = Color32::from_rgb(0, 255, 0);
    }
}

static mut MEMORY_STATE: MemoryState = MemoryState {
    descriptor: None,
    guard: None,
    current_value: String::new(),
};

static mut PROCESS_STATE: ProcessState = ProcessState {
    process_id_text: String::new(),
    current_process: None,
    current_process_path: None,
};

static mut GENERAL_STATE: GeneralState = GeneralState {
    guid_text: String::new(),
};

#[derive(Default)]
enum AppState {
    #[default]
    None,
    General(&'static mut GeneralState),
    Process(&'static mut ProcessState),
    Memory(&'static mut MemoryState),
}

struct ProcessState {
    current_process: Option<HxProcess>,
    process_id_text: String,
    current_process_path: Option<String>,
}

struct GeneralState {
    guid_text: String,
}

struct MemoryState {
    descriptor: Option<HxMemoryDescriptor<u64>>,
    guard: Option<HxMemoryGuard<'static, u64>>,
    current_value: String
}

impl Default for GeneralState {
    fn default() -> Self {
        Self {
            guid_text: "ca170835-4a59-4c6d-a04b-f5866f592c38".to_owned(),
        }
    }
}

impl eframe::App for HxTestApp {
    #[allow(static_mut_refs)]
    fn update(&mut self, ui: &egui::Context, _frame: &mut eframe::Frame) {
        let mut error_update: Option<String> = None;
        let mut ok_update: Option<String> = None;

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("HxTest");
            ui.horizontal(|ui| {
                ui.colored_label(self.status_label_color, &mut self.status_label_text);
            });
            ui.horizontal(|ui| unsafe {
                if ui.button("General").clicked() {
                    self.state = AppState::General(&mut GENERAL_STATE);
                }
                if ui.button("Process").clicked() {
                    self.state = AppState::Process(&mut PROCESS_STATE);
                }
                if ui.button("Threads").clicked() {}
                if ui.button("Memory").clicked() {
                    self.state = AppState::Memory(&mut MEMORY_STATE);
                }
            });
            ui.vertical(|ui| match &mut self.state {
                AppState::None => {}
                AppState::Memory(state) => {
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Allocate").clicked() {
                            match async_std::task::block_on(HxMemory::alloc::<u64>(
                                MemoryPool::NonPaged,
                            )) {
                                Ok(x) => {
                                    ok_update = Some(format!("Successfully allocated: {:?}", x));
                                    state.descriptor = Some(x);
                                }
                                Err(err) => {
                                    error_update =
                                        Some(format!("Error allocating memory: {:?}", err));
                                }
                            };
                        }
                        if ui.button("Free").clicked() {
                            let mem = state.descriptor.take().unwrap();
                            drop(mem);
                            ok_update = Some("Successfully freed".to_string());
                        }
                    });

                    if state.descriptor.is_none() {
                        return;
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Map").clicked() {
                            let result = unsafe {
                                let desc = state.descriptor.as_mut().unwrap();
                                async_std::task::block_on(desc.map(
                                    PROCESS_STATE.current_process.as_mut(),
                                    None
                                ))
                            };

                            match result {
                                Ok(guard) => {
                                    let static_guard: HxMemoryGuard<'static, u64> = unsafe {
                                        std::mem::transmute(guard)
                                    };

                                    ok_update = Some(format!("Successfully mapped: {:?}", static_guard));
                                    state.guard = Some(static_guard);
                                }
                                Err(err) => {
                                    error_update = Some(format!("Error mapping memory: {:?}", err));
                                }
                            };
                        }
                        if ui.button("Unmap").clicked() {
                            let result = {
                                let desc = state.guard.as_mut().unwrap();
                                async_std::task::block_on(desc.unmap())
                            };

                            match result {
                                Ok(x) => {
                                    ok_update = Some(format!("Successfully unmapped: {:?}", x));
                                    drop(state.guard.take());
                                }
                                Err(err) => {
                                    error_update = Some(format!("Error mapping memory: {:?}", err));
                                }
                            }
                        }
                    });
                    if state.guard.is_none() {
                        return;
                    }
                    ui.separator();
                    ui.text_edit_singleline(&mut state.current_value);
                    ui.horizontal(|ui| {
                        if ui.button("Read").clicked() {
                            state.current_value = state.guard.as_mut().unwrap().clone().to_string();
                        }
                        if ui.button("Write").clicked() {
                            *state.guard.as_mut().unwrap().deref_mut() = u64::from_str(state.current_value.as_str()).unwrap();
                        }
                    });
                }
                AppState::General(state) => {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Plugin GUID:");
                        ui.text_edit_singleline(&mut state.guid_text);
                        if ui.button("Authorize").clicked() {
                            let uuid = match Uuid::from_str(&state.guid_text) {
                                Ok(x) => x,
                                Err(err) => {
                                    error_update = Some(format!("Error parsing GUID: {:?}", err));
                                    return;
                                }
                            };

                            match AuthorizationRequest::new(uuid, PluginPermissions::all()).send() {
                                Ok(x) => {
                                    ok_update = Some(format!("Successfully authorized: {:?}", x));
                                }
                                Err(err) => {
                                    error_update = Some(format!("Error authorizing: {:?}", err));
                                }
                            };
                        }
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Get status").clicked() {
                            match StatusRequest::default().send() {
                                Ok(x) => {
                                    ok_update = Some(format!("Hypervisor status: {:?}", x));
                                }
                                Err(err) => {
                                    error_update = Some(format!("Error getting status: {:?}", err));
                                }
                            }
                        }
                    });
                }
                AppState::Process(state) => {
                    ui.separator();
                    let current_pid = match &state.current_process {
                        None => 0,
                        Some(x) => x.id,
                    };
                    ui.label(format!("Current process ID: {}", current_pid));
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Process ID:");
                        ui.text_edit_singleline(&mut state.process_id_text);
                        if ui.button("Open").clicked() {
                            let pid = match u32::from_str(state.process_id_text.as_str()) {
                                Ok(x) => x,
                                Err(err) => {
                                    error_update =
                                        Some(format!("Error parsing process id: {:?}", err));
                                    return;
                                }
                            };

                            let req = match HxProcess::open(pid) {
                                Ok(x) => {
                                    ok_update = Some(format!("Successfully opened: {:?}", x));
                                    x
                                }
                                Err(err) => {
                                    error_update =
                                        Some(format!("Error opening process: {:?}", err));
                                    return;
                                }
                            };

                            state.current_process = Some(req);
                        }
                    });
                    ui.separator();
                    if state.current_process.is_none() {
                        return;
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Kill").clicked() {
                            let process = state.current_process.take().unwrap();
                            match async_std::task::block_on(process.kill(0)) {
                                Ok(_) => {
                                    ok_update = Some("Successfully killed process".to_string());
                                    drop(process);
                                }
                                Err(err) => {
                                    error_update =
                                        Some(format!("Error killing process: {:?}", err));
                                }
                            }
                        }
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        match state.current_process_path {
                            None => {
                                match async_std::task::block_on(
                                    state.current_process.as_mut().unwrap().get_nt_path(),
                                ) {
                                    Ok(x) => {
                                        state.current_process_path = Some(x);
                                    }
                                    Err(err) => {
                                        error_update =
                                            Some(format!("Error killing process: {:?}", err));
                                    }
                                }
                            }
                            Some(_) => {}
                        }

                        ui.text_edit_singleline(state.current_process_path.as_mut().unwrap());
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Protect process").clicked() {
                            match async_std::task::block_on(
                                state.current_process.as_mut().unwrap().set_protection(
                                    ProcessProtection::new()
                                        .with_protection_type(ProtectionType::Protected)
                                        .with_audit(false)
                                        .with_signer(ProtectionSigner::AntiMalware),
                                ),
                            ) {
                                Ok(_) => {
                                    ok_update = Some("Successfully protected process".to_string());
                                }
                                Err(err) => {
                                    error_update =
                                        Some(format!("Error protecting process: {:?}", err));
                                }
                            }
                        }
                        if ui.button("Unprotect process").clicked() {
                            match async_std::task::block_on(
                                state.current_process.as_mut().unwrap().set_protection(
                                    ProcessProtection::new()
                                        .with_protection_type(ProtectionType::None)
                                        .with_audit(false)
                                        .with_signer(ProtectionSigner::AntiMalware),
                                ),
                            ) {
                                Ok(_) => {
                                    ok_update =
                                        Some("Successfully unprotected process".to_string());
                                }
                                Err(err) => {
                                    error_update =
                                        Some(format!("Error unprotecting process: {:?}", err));
                                }
                            }
                        }
                    });
                }
            })
        });

        if let Some(msg) = error_update {
            self.set_error(msg);
        } else if let Some(msg) = ok_update {
            self.set_ok(msg);
        }
    }
}
