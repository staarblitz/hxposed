mod anti_borrow;

use std::cell::{RefCell, UnsafeCell};
use async_std::prelude::*;
use hxposed_core::hxposed::requests::Vmcall;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::status::StatusRequest;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::memory::HxMemory;
use hxposed_core::services::memory_map::{HxMemoryDescriptor, HxMemoryGuard};
use hxposed_core::services::process::HxProcess;
use hxposed_core::services::types::memory_fields::{MemoryPool, MemoryProtection};
use std::fmt::Display;
use std::mem;
use std::ptr::null_mut;
use std::str::FromStr;
use uuid::Uuid;
use crate::anti_borrow::ExtremeCell;

async fn general_category() {
    println!("General:");
    println!("[0] - Exit");
    println!("[1] - Status");
    println!("[2] - Authorize");

    loop {
        let mut command = String::new();

        async_std::io::stdin()
            .read_line(&mut command)
            .await
            .unwrap();

        match command.trim() {
            "0" => {
                return;
            }
            "1" => {
                println!("{:?}", StatusRequest {}.send());
            }
            "2" => {
                println!(
                    "{:?}",
                    AuthorizationRequest {
                        uuid: Uuid::from_str("ca170835-4a59-4c6d-a04b-f5866f592c38").unwrap(),
                        permissions: PluginPermissions::all()
                    }
                    .send()
                );
            }
            _ => {
                println!("Please enter a valid command.");
            }
        }
    }
}

async fn process_category() {
    let mut process: Option<HxProcess> = None;

    println!("Process:");
    println!("[0] - Exit");
    println!("[1] - Open");
    println!("[2] - Get primary token");
    println!("[3] - Swap token");
    println!("[4] - Get mitigation options");
    println!("[5] - Set mitigation options");
    println!("[6] - Get threads");
    println!("[7] - Get protection");
    println!("[8] - Set protection");
    println!("[9] - Set signature levels");
    println!("[10] - Get signature levels");
    println!("[11] - Get nt path");
    println!("[12] - Kill");

    loop {
        let mut command = String::new();
        let mut param = String::new();

        async_std::io::stdin()
            .read_line(&mut command)
            .await
            .unwrap();

        match command.trim() {
            "0" => {
                return;
            }
            "1" => {
                println!("Which proceess?");
                async_std::io::stdin().read_line(&mut param).await.unwrap();

                let process_id = u32::from_str(param.trim_end()).unwrap();

                let proc = HxProcess::open(process_id);

                println!("{:?}", proc);

                if let Ok(proc) = proc {
                    process = Some(proc);
                }
            }
            _ => {
                let process = match process {
                    Some(ref mut proc) => proc,
                    None => {
                        println!("Open a process first");
                        break;
                    }
                };

                match command.trim() {
                    "2" => {
                        let tkn = process.get_primary_token().await;
                        println!("{:?}", tkn);

                        match tkn {
                            Ok(_) => {}
                            Err(err) => {
                                println!("Failed to open token: {}", err);
                                break;
                            }
                        }
                    }
                    "3" => {
                        println!("Enter process id of new token to swap:");
                        async_std::io::stdin().read_line(&mut param).await.unwrap();

                        let process_id = u32::from_str(param.trim_end()).unwrap();

                        let proc = match HxProcess::open(process_id) {
                            Ok(proc) => proc,
                            Err(e) => {
                                println!("Failed to open process: {}", e);
                                break;
                            }
                        };

                        let new_token = match proc.get_primary_token().await {
                            Ok(x) => x,
                            Err(e) => {
                                println!("Failed to get primary token: {}", e);
                                break;
                            }
                        };

                        match process.swap_token(&new_token).await {
                            Ok(_) => {
                                println!("Tokens swapped!")
                            }
                            Err(e) => {
                                println!("Failed to swap token: {}", e);
                                break;
                            }
                        };
                    }
                    _ => {
                        println!("Please enter a valid command.");
                    }
                }
            }
        }
    }
}

async fn memory_category<'a>() {
    let global_memory: ExtremeCell<Option<HxMemoryDescriptor<u64>>> = ExtremeCell::new(None);
    let guard: ExtremeCell<Option<HxMemoryGuard<'a, u64>>> = ExtremeCell::new(None);

    println!("Memory:");
    println!("[0] - Exit");
    println!("[1] - Allocate");
    println!("[2] - Map");
    println!("[3] - Read");
    println!("[4] - Write");
    println!("[5] - Protect");
    println!("[6] - Unmap");
    println!("[7] - Free");

    'begin: loop {
        let mut command = String::new();
        let mut param = String::new();

        async_std::io::stdin()
            .read_line(&mut command)
            .await
            .unwrap();

        match command.trim() {
            "0" => {
                return;
            }
            "1" => match HxMemory::alloc::<u64>(MemoryPool::NonPaged).await {
                Ok(x) => {
                    global_memory.replace(Some(x));
                }
                Err(err) => {
                    println!("Failed to allocate memory: {}", err);
                }
            },
            _ => {
                let mem = global_memory.as_mut().as_mut().unwrap();

                match command.trim() {
                    "2" => {
                        println!("To which process? Type 0 for current.");

                        // this should be shorter smh
                        async_std::io::stdin().read_line(&mut param).await.unwrap();

                        let pid = u32::from_str(param.trim_end()).unwrap();

                        let process = match pid {
                            0 => None,
                            _ => Some(&mut match HxProcess::open(pid) {
                                Ok(mut x) => x,
                                Err(err) => {
                                    println!("Failed to open process: {}", err);
                                    break 'begin;
                                }
                            }),
                        };

                        match mem.map(process, None).await {
                            Ok(x) => {
                                guard.replace(Some(x));
                            }
                            Err(err) => {
                                println!("Failed to open process: {}", err);
                                break 'begin;
                            }
                        }
                    }
                    "5" => {
                        println!("Of which process? Type 0 for current.");
                        async_std::io::stdin().read_line(&mut param).await.unwrap();

                        let pid = u32::from_str(param.trim_end()).unwrap();

                        let process = match pid {
                            0 => &mut HxProcess::current().unwrap(),
                            _ => &mut match HxProcess::open(pid) {
                                Ok(x) => x,
                                Err(err) => {
                                    println!("Failed to open process: {}", err);
                                    break 'begin;
                                }
                            },
                        };

                        println!("Of which address (hex)?");
                        async_std::io::stdin().read_line(&mut param).await.unwrap();
                        let addr = usize::from_str_radix(param.trim_end(), 16).unwrap();

                        match process
                            .memory
                            .protect(addr as _, MemoryProtection::EXECUTE_READWRITE)
                            .await
                        {
                            Ok(x) => {
                                println!("Protected! Old protection: {:?}", x);
                            }
                            Err(err) => {
                                println!("Failed to protect memory: {}", err);
                                break 'begin;
                            }
                        }
                    }
                    "6" => {}
                    "7" => {
                        guard.drop_inner();
                    }
                    _ => {

                        let guard = guard.as_mut().as_mut().unwrap();

                        match command.trim() {
                            "3" => {
                                println!("Value: {}", **guard);
                            }
                            "4" => {
                                println!("What to write?");

                                async_std::io::stdin().read_line(&mut param).await.unwrap();

                                // well, this is another way to get the job done.
                                **guard = param.trim_end().parse::<u64>().unwrap();
                            }
                            _ => {
                                println!("Please enter a valid command.");
                            }
                        }
                    }
                }
            }
        }
    }
}

fn print_categories() {
    println!("Categories:");
    println!("[0] - General");
    println!("[1] - Process");
    println!("[2] - Thread");
    println!("[3] - Security");
    println!("[4] - Memory");
}

async fn async_main() {
    println!("Welcome to HxTest!");
    println!("This utility tests various functions of HxPosed.");
    println!("Select a category to begin....");

    print_categories();

    loop {
        let mut command = String::new();

        async_std::io::stdin()
            .read_line(&mut command)
            .await
            .unwrap();

        match command.trim() {
            "0" => {
                general_category().await;
            }
            "1" => {
                process_category().await;
            }
            "4" => {
                memory_category().await;
            }
            _ => {
                println!("Please enter a valid category.");
            }
        }

        print_categories();
    }
}

fn main() {
    async_std::task::block_on(async_main());
}

fn print_hex_grid(data: &[u8]) {
    let cols = 16;

    for (i, byte) in data.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % cols == 0 {
            println!();
        }
    }
    if data.len() % cols != 0 {
        println!();
    }
}
