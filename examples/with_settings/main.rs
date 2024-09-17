use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy_registry_exporter::prelude::{ExportRegistryPlugin, ExportRegistrySettings};

fn main() -> AppExit {
    // Check if target path exists and create it if not.
    let path = String::from("assets/test");
    let dir = PathBuf::from(&path);
    if !dir.clone().exists() {
        std::fs::create_dir_all(dir).unwrap();
    }

    // Build target file name.
    let dir = PathBuf::from(path);
    let save_path = Path::join(&dir, "registry.json");
    let settings: ExportRegistrySettings = ExportRegistrySettings {
        resource_filter : SceneFilter::default(),
        component_filter : SceneFilter::default(),
        save_path,
    };

    App::new()
        // Add the settings as a resource
        .insert_resource(settings)
        
        .add_plugins(
            (
                DefaultPlugins, 
                ExportRegistryPlugin {}
            ))
        .run()
}
