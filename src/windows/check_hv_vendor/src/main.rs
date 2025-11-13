use std::str::FromStr;
use hxposed_core::hxposed::requests::Vmcall;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::status::StatusRequest;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use uuid::Uuid;

fn main() {
    println!("Preparing status request...");
    let req = StatusRequest::default();
    println!("Status request: {:?}", req);
    let resp = req.send();
    match resp {
        Err(e) => {
            println!("Error status request! {:?}", e);
        }
        _ => {}
    }

    let resp = unsafe { resp.unwrap() };

    println!(
        "Hypervisor status: Current: {}, Version: {}",
        resp.state, resp.version
    );

    let uuid = Uuid::from_str("ca170835-4a59-4c6d-a04b-f5866f592c38").unwrap();
    println!("Authorizing with UUID {}", uuid);
    let req = AuthorizationRequest {
        permissions: PluginPermissions::all(),
        uuid,
    };

    let resp = req.send();
    match resp {
        Err(e) => {
            println!("Error authorization request! {:?}", e);
        }
        _ => {}
    }

    let resp = unsafe { resp.unwrap_unchecked() };

    println!("Permissions: {:?}", resp.permissions);
}
