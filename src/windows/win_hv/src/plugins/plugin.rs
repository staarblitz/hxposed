use hxposed_core::plugins::plugin_perms::PluginPermissions;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct Plugin {
    pub uuid: Uuid,
    pub permissions: PluginPermissions,
}

impl Plugin {
    pub fn open(uuid: Uuid) -> Self {

    }
}
