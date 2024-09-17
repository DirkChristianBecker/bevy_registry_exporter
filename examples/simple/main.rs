use bevy::prelude::*;
use bevy_registry_exporter::prelude::ExportRegistryPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(
            (
                DefaultPlugins, 
                ExportRegistryPlugin {}
            ))
        .run()
}
