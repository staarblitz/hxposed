use hxposed_core::hxposed::requests::Vmcall;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::status::StatusRequest;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::memory::HxMemory;
use hxposed_core::services::types::memory_fields::{MemoryPool, MemoryProtection};
use std::fmt::Display;
use std::process::exit;
use std::str::FromStr;
use async_std::io::stdin;
use hxposed_core::error::HypervisorError;
use hxposed_core::hxposed::requests::process::ProcessField::MitigationFlags;
use hxposed_core::services::process::HxProcess;
use hxposed_core::services::thread::HxThread;
use hxposed_core::services::types::process_fields::{MitigationOptions, ProcessProtection, ProcessSignatureLevel, ProcessSignatureLevels, ProtectionSigner, ProtectionType};
use uuid::Uuid;

async fn async_main() {
    let perms = PluginPermissions::all();

    let uuid = Uuid::from_str("ca170835-4a59-4c6d-a04b-f5866f592c38").unwrap();
    println!("Authorizing with UUID {}", uuid);

    let result = AuthorizationRequest::new(uuid, PluginPermissions::all()).send();

    if let Err(e) = result {
        println!("Error authorizing: {:?}", e);
        return;
    }

    println!("Permissions: {:?}", result.unwrap().permissions);

    println!("Getting status");

    let req = StatusRequest::default();
    let resp = req.send();
    match resp {
        Err(e) => {
            println!("Error status request! {:?}", e);
        }
        _ => {}
    }

    let resp = resp.unwrap();

    println!(
        "Hypervisor status: Current: {}, Version: {}",
        resp.state, resp.version
    );

    println!("Process id to open?: ");
    let mut input = String::new();
    stdin().read_line(&mut input).await.unwrap();

    let id: u32 = input.trim_end().parse().unwrap();

    println!("Trying to open a process...");
    let mut process = match HxProcess::open(id) {
        Ok(x) => x,
        Err(e) => {
            println!("Error opening process: {:?}", e);
            return;
        }
    };

    println!("Opened process!");

    let path = match process.get_nt_path().await {
        Ok(x) => x,
        Err(e) => {
            println!("Error getting nt path of process: {:?}", e);
            return;
        }
    };

    println!("NT path of the process object: {}", path);

    let protection = match process.get_protection() {
        Ok(x) => x,
        Err(e) => {
            println!("Error getting process protection: {:?}", e);
            return;
        }
    };

    println!("Process protection: {:?}", protection);

    let signature_levels = match process.get_signature_levels() {
        Ok(x) => x,
        Err(e) => {
            println!("Error getting process signer levels: {:?}", e);
            return;
        }
    };

    println!("Process signature levels: {:?}", signature_levels);

    let options = match process.get_mitigation_options().await {
        Ok(x) => x,
        Err(e) => {
            println!("Error getting process mitigation levels: {:?}", e);
            return;
        }
    };

    println!("Process mitigation options: {:?}", options);

    match process
        .set_protection(
            ProcessProtection::new()
                .with_audit(false)
                .with_protection_type(ProtectionType::None)
                .with_signer(ProtectionSigner::None),
        )
        .await
    {
        Ok(_) => println!("Process protection changed!"),
        Err(x) => println!("Error changing process protection: {:?}", x),
    }

    match process
        .set_signature_levels(
            ProcessSignatureLevels::new()
                .with_signature_level(ProcessSignatureLevel::AntiMalware)
                .with_section_signature_level(0),
        )
        .await
    {
        Ok(_) => println!("Process signature levels changed!"),
        Err(x) => println!("Error changing process signature levels: {:?}", x),
    }

    match process.set_mitigation_options(MitigationOptions::default()).await {
        Ok(_) => println!("Process mitigation options changed!"),
        Err(x) => println!("Error changing process mitigation options: {:?}", x),
    }

    /*println!("Address to read/write?: ");
    let mut input = String::new();
    stdin().read_line(&mut input).await.unwrap();

    let addr = u64::from_str_radix(&input.trim(), 16).unwrap();

    let vec = match process.memory.read::<u8>(addr as *mut u8, 256).await {
        Ok(x) => {
            println!("Read {} bytes from the process", x.len());
            x
        }
        Err(e) => {
            println!("Error reading from process memory: {:?}", e);
            return;
        }
    };

    println!("BEGIN {}  ============================", addr);
    print_hex_grid(&vec);
    println!("END {}  ============================", addr + vec.len() as u64);

    match process.memory.protect(addr as _, MemoryProtection::READWRITE).await {
        Ok(x) => println!("Protected read-only memory. Old protection: {:?}", x),
        Err(e) => println!("Error changing memory protection: {:?}", e),
    }

    let mut zeros: [u8; 256] = [0; 256];

    match process.memory.write::<u8>(addr as _, zeros.as_mut_ptr() as _, zeros.len()).await {
        Ok(x) => {
            println!("Written {} bytes to the process", x)
        }
        Err(e) => {
            println!("Error writing to process memory: {:?}", e);
            return;
        }
    }*/

/*    println!("Allocating kernel memory....");

    let mut allocation = match HxMemory::alloc::<u64>(MemoryPool::NonPaged).await {
        Ok(x) => x,
        Err(e) => {
            println!("Error allocating kernel memory: {:?}", e);
            return;
        }
    };

    println!("Memory allocated!");

    {
        let mut _guard = match allocation.map(None).await {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to map memory: {:?}", e);
                return;
            }
        };

        println!("Memory mapped!");

        *_guard = 5;
    }

    {
        let mut _guard = match allocation.map(None).await {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to map memory: {:?}", e);
                return;
            }
        };

        println!("Previous value: {}", *_guard);
        assert_eq!(*_guard, 5);
    }

    allocation.free();*/

    // println!("Sending command to kill process...");
    //
    // match process.kill(0).await {
    //     Ok(_) => {
    //         println!("Killed process!");
    //     }
    //     Err(e) => {
    //         println!("Error killing process: {:?}", e);
    //     }
    // }
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
