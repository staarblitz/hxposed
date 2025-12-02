use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::status::StatusRequest;
use hxposed_core::hxposed::requests::{Vmcall, VmcallRequest};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::process::HxProcess;
use std::io::stdin;
use std::str::FromStr;
use std::task;
use hxposed_core::error::HypervisorError;
use hxposed_core::hxposed::error::{ErrorSource, InternalErrorCode};
use uuid::Uuid;

async fn async_main() {
    let uuid = Uuid::from_str("ca170835-4a59-4c6d-a04b-f5866f592c38").unwrap();
    println!("Authorizing with UUID {}", uuid);

    let resp = match AuthorizationRequest::new(uuid, PluginPermissions::all()).send() {
        Err(e) => {
            println!("Error authorization request! {:?}", e);
            return;
        }
        Ok(x) => x,
    };

    println!("Permissions: {:?}", resp.permissions);

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
    stdin().read_line(&mut input).unwrap();

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

    println!("Sending command to kill process...");

    match process.kill(0).await {
        Ok(_) => {
            println!("Killed process!");
        }
        Err(e) => {
            println!("Error killing process: {:?}", e);
        }
    }
}

fn main() {
  async_std::task::block_on(async_main());
}
