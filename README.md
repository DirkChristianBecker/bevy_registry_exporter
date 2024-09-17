# bevy_registry_exporter
This crate exports the bevy registry as a json file, when added to a bevy app. It will export the registry, only if the program was build in debug mode. In release mode this plugin will not register any types or methods to run. However, if not prevented by the hosting application, the plugin itself will be add to the bevy app. 

## Export settings
The exporter can be controlled by adding an instance of the type 'ExportRegistrySettings'. This way we can control, which components and resources should be part of the export and which should not be. 
On top of that an export directory can be defined. By default the file with the exported registry will be called 'registry.json' and will be placed inside the assets directory of the application. However, the path of the registry can be freely defined (as shown in the 'with_settings' - Example).
When no export settings are defined the assets directory will be created, if it does not exist yet. When using a non-default directory the user is in charge of maintaining the target directory (which means the application will crash if not done properly).
