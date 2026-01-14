use egui::{Color32, ViewportBuilder};
use hxposed_core::error::HypervisorError;
use hxposed_core::hxposed::requests::status::StatusRequest;
use hxposed_core::hxposed::requests::Vmcall;
use hxposed_core::hxposed::responses::notify::AwaitNotificationResponse;
use hxposed_core::hxposed::ObjectType;
use hxposed_core::services::callbacks::HxCallback;
use hxposed_core::services::memory::HxMemory;
use hxposed_core::services::memory_map::{HxMemoryDescriptor, HxMemoryGuard};
use hxposed_core::services::process::HxProcess;
use hxposed_core::services::types::memory_fields::MemoryPool;
use hxposed_core::services::types::process_fields::{
    ProcessProtection, ProtectionSigner, ProtectionType,
};
use std::borrow::ToOwned;
use std::mem;
use std::ops::DerefMut;
use std::str::FromStr;
use std::sync::Mutex;

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

static mut CALLBACKS_STATE: CallbacksState = CallbacksState {
    events: Mutex::new(Vec::new()),
    cback2: None,
    cback1: None,
};

static mut MEMORY_STATE: MemoryState = MemoryState {
    descriptor: None,
    guard: None,
    current_value: String::new(),
};

static mut PROCESS_STATE: ProcessState = ProcessState {
    process_id_text: String::new(),
    current_process: None,
    current_process_path: None,
    protection_type: ProtectionType::None,
    protection_audit: false,
    protection_signer: ProtectionSigner::None,
};

#[derive(Default)]
enum AppState {
    #[default]
    General,
    Process(&'static mut ProcessState),
    Memory(&'static mut MemoryState),
    Callbacks(&'static mut CallbacksState),
}

struct CallbacksState {
    events: Mutex<Vec<String>>,
    cback1: Option<HxCallback>,
    cback2: Option<HxCallback>,
}

struct ProcessState {
    current_process: Option<HxProcess>,
    process_id_text: String,
    current_process_path: Option<String>,
    protection_type: ProtectionType,
    protection_audit: bool,
    protection_signer: ProtectionSigner,
}

struct GeneralState {
    guid_text: String,
}

struct MemoryState {
    descriptor: Option<HxMemoryDescriptor<u64>>,
    guard: Option<HxMemoryGuard<'static, u64>>,
    current_value: String,
}

impl Default for GeneralState {
    fn default() -> Self {
        Self {
            guid_text: "ca170835-4a59-4c6d-a04b-f5866f592c38".to_owned(),
        }
    }
}

fn log_callback(x: Result<AwaitNotificationResponse, HypervisorError>) {
    match x {
        Ok(x) => {
            println!(
                "Event for object {:?}, with state of {:?}",
                x.object_type, x.object_state
            );
        }
        Err(err) => {
            println!("Callback returned error: {:?}", err);
            return;
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
                    self.state = AppState::General;
                }
                if ui.button("Process").clicked() {
                    self.state = AppState::Process(&mut PROCESS_STATE);
                }
                if ui.button("Threads").clicked() {}
                if ui.button("Memory").clicked() {
                    self.state = AppState::Memory(&mut MEMORY_STATE);
                }
                if ui.button("Callbacks").clicked() {
                    self.state = AppState::Callbacks(&mut CALLBACKS_STATE);
                }
            });
            ui.vertical(|ui| {
                ui.separator();
                match &mut self.state {
                    AppState::Callbacks(state) => {
                        ui.horizontal(|ui| {
                            if ui.button("Register callbacks").clicked() {
                                let cback1 = match HxCallback::new(
                                    Box::new(log_callback),
                                    ObjectType::Process(0),
                                ) {
                                    Ok(x) => x,
                                    Err(err) => {
                                        error_update =
                                            Some(format!("Error registering callback: {:?}", err));
                                        return;
                                    }
                                };

                                let cback2 = match HxCallback::new(
                                    Box::new(log_callback),
                                    ObjectType::Thread(0),
                                ) {
                                    Ok(x) => x,
                                    Err(err) => {
                                        error_update =
                                            Some(format!("Error registering callback: {:?}", err));
                                        return;
                                    }
                                };

                                state.cback1 = Some(cback1);
                                state.cback2 = Some(cback2);

                                let cback1: &'static HxCallback =
                                    unsafe { mem::transmute(state.cback1.as_ref().unwrap()) };

                                let cback2: &'static HxCallback =
                                    unsafe { mem::transmute(state.cback1.as_ref().unwrap()) };

                                async_std::task::spawn(async {
                                    println!("Beginning event loop on cback1!");
                                    cback1.event_loop().await;
                                });

                                async_std::task::spawn(async {
                                    println!("Beginning event loop on cback2!");
                                    cback2.event_loop().await;
                                });
                            }
                            if ui.button("Unregister callbacks").clicked() {
                                drop(state.cback2.take().unwrap());
                            }
                        });
                        ui.separator();
                        ui.text_edit_multiline(&mut state.events.lock().unwrap().join("\n"));
                    }
                    AppState::Memory(state) => {
                        ui.horizontal(|ui| {
                            if ui.button("Allocate").clicked() {
                                match async_std::task::block_on(HxMemory::alloc::<u64>(
                                    MemoryPool::NonPaged,
                                )) {
                                    Ok(x) => {
                                        ok_update =
                                            Some(format!("Successfully allocated: {:?}", x));
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
                                    async_std::task::block_on(
                                        desc.map(PROCESS_STATE.current_process.as_mut(), None),
                                    )
                                };

                                match result {
                                    Ok(guard) => {
                                        let static_guard: HxMemoryGuard<'static, u64> =
                                            unsafe { mem::transmute(guard) };

                                        ok_update = Some(format!(
                                            "Successfully mapped: {:?}",
                                            static_guard
                                        ));
                                        state.guard = Some(static_guard);
                                    }
                                    Err(err) => {
                                        error_update =
                                            Some(format!("Error mapping memory: {:?}", err));
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
                                        error_update =
                                            Some(format!("Error mapping memory: {:?}", err));
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
                                state.current_value =
                                    state.guard.as_mut().unwrap().clone().to_string();
                            }
                            if ui.button("Write").clicked() {
                                *state.guard.as_mut().unwrap().deref_mut() =
                                    u64::from_str(state.current_value.as_str()).unwrap();
                            }
                        });
                    }
                    AppState::General => {
                        ui.horizontal(|ui| {
                            if ui.button("Get status").clicked() {
                                match StatusRequest::default().send() {
                                    Ok(x) => {
                                        ok_update = Some(format!("Hypervisor status: {:?}", x));
                                    }
                                    Err(err) => {
                                        error_update =
                                            Some(format!("Error getting status: {:?}", err));
                                    }
                                }
                            }
                        });
                    }
                    AppState::Process(state) => {
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

                        ui.group(|ui| {
                            ui.columns(2, |columns| {
                                columns[0].label("Protection type: ");
                                columns[0].label("Is audit: ");
                                columns[0].label("Protection signer: ");

                                egui::ComboBox::from_label("X")
                                    .selected_text(format!("{:?}", state.protection_type))
                                    .show_ui(&mut columns[1], |ui| {
                                        ui.selectable_value(
                                            &mut state.protection_type,
                                            ProtectionType::None,
                                            "None",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_type,
                                            ProtectionType::Light,
                                            "Light",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_type,
                                            ProtectionType::Protected,
                                            "Protected",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_type,
                                            ProtectionType::Max,
                                            "Max",
                                        );
                                    });
                                columns[1].checkbox(&mut state.protection_audit, "Audit");
                                egui::ComboBox::from_label("Y")
                                    .selected_text(format!("{:?}", state.protection_signer))
                                    .show_ui(&mut columns[1], |ui| {
                                        ui.selectable_value(
                                            &mut state.protection_signer,
                                            ProtectionSigner::None,
                                            "None",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_signer,
                                            ProtectionSigner::Authenticode,
                                            "Authenticode",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_signer,
                                            ProtectionSigner::CodeGen,
                                            "CodeGen",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_signer,
                                            ProtectionSigner::AntiMalware,
                                            "AntiMalware",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_signer,
                                            ProtectionSigner::Lsa,
                                            "Lsa",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_signer,
                                            ProtectionSigner::Windows,
                                            "Windows",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_signer,
                                            ProtectionSigner::WinTcb,
                                            "WinTcb",
                                        );
                                        ui.selectable_value(
                                            &mut state.protection_signer,
                                            ProtectionSigner::Max,
                                            "Max",
                                        );
                                    });
                            });

                            ui.horizontal(|ui| {
                                if ui.button("Get protection").clicked() {
                                    let protection = match state
                                        .current_process
                                        .as_mut()
                                        .unwrap()
                                        .get_protection()
                                    {
                                        Ok(x) => x,
                                        Err(err) => {
                                            error_update = Some(format!(
                                                "Error getting process protection: {:?}",
                                                err
                                            ));
                                            return;
                                        }
                                    };

                                    state.protection_audit = protection.audit();
                                    state.protection_signer = protection.signer();
                                    state.protection_type = protection.protection_type();
                                }
                                if ui.button("Set protection").clicked() {
                                    let protection = ProcessProtection::new()
                                        .with_protection_type(state.protection_type)
                                        .with_signer(state.protection_signer)
                                        .with_audit(state.protection_audit);
                                    match async_std::task::block_on(
                                        state
                                            .current_process
                                            .as_mut()
                                            .unwrap()
                                            .set_protection(protection),
                                    ) {
                                        Ok(_) => {
                                            ok_update =
                                                Some("Successfully protected process".to_string());
                                        }
                                        Err(err) => {
                                            error_update = Some(format!(
                                                "Error protecting process: {:?}",
                                                err
                                            ));
                                        }
                                    }
                                }
                            });
                        });
                    }
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
