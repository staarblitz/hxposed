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

    let resp = unsafe { resp.unwrap_unchecked() };

    println!(
        "Hypervisor status: Current: {}, Version: {}",
        resp.state, resp.version
    );

    let uuid = Uuid::from_u64_pair(0, 0);
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
