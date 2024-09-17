use std::path::{Path, PathBuf};

#[cfg(debug_assertions)]
use bevy::{
    app::Startup,
    asset::AssetPlugin,
    prelude::{App, Plugin, Resource}, scene::SceneFilter,
};

#[cfg(not(debug_assertions))]
use bevy::{
    prelude::{App, Plugin, Resource}, scene::SceneFilter,
};

#[cfg(debug_assertions)]
use crate::export_types::export_types;

/// Export settings can be used to filter certain resources or components
/// and set an export path for the registry. If no export settings are provided
/// the default settings are beeing used. Which means no filter for either components
/// or resources and the export path will be assets/registry.json.
/// If the defaults are beeing use, the export will create the assets directory if 
/// it does not exist yet. Ohterwise the directory to store the registry in 
/// must exist before hand. 
#[derive(Resource)]
pub struct ExportRegistrySettings {
    pub component_filter: SceneFilter,
    pub resource_filter: SceneFilter,
    pub save_path: PathBuf,
}

impl Default for ExportRegistrySettings {
    fn default() -> Self {
        Self 
        { 
            component_filter: SceneFilter::default(), 
            resource_filter: SceneFilter::default(), 
            save_path: Path::join(&PathBuf::from("assets"), "registry.json"),
        }
    }
}

/// This Plugin will export the entire registry on startup, if bevy was build with debug_assertions.
pub struct ExportRegistryPlugin { }

#[cfg(debug_assertions)]
const ASSET_ERROR: &str = "Bevy_registry_export requires access to the Bevy asset plugin. \
    Please add `ExportRegistryPlugin` after `AssetPlugin`, which is commonly added as part of the `DefaultPlugins`";

impl Plugin for ExportRegistryPlugin {
    #[cfg(debug_assertions)]
    fn build(&self, app: &mut App) {
            let asset_plugins: Vec<&AssetPlugin> = app.get_added_plugins();
            let asset_plugin = asset_plugins.into_iter().next().expect(ASSET_ERROR);
            let path_str = asset_plugin.file_path.clone();
            let path = PathBuf::from(path_str);

            app.insert_resource(AssetRoot(path));
            app.add_systems(Startup, export_types);
    }

    #[cfg(not(debug_assertions))]
    fn build(&self, _app: &mut App) {

    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub(crate) struct AssetRoot(pub(crate) PathBuf);
