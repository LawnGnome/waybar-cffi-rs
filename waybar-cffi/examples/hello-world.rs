use serde::Deserialize;
use waybar_cffi::{
    gtk::{prelude::ContainerExt, Label},
    waybar_module, InitInfo, Module,
};

struct HelloWorld;

impl Module for HelloWorld {
    type Config = Config;

    fn init(info: &InitInfo, config: Config) -> Self {
        let container = info.get_root_widget();
        let label = Label::new(Some(&format!(
            "Hello {}!",
            config.name.as_deref().unwrap_or("World")
        )));
        container.add(&label);

        HelloWorld
    }
}

waybar_module!(HelloWorld);

#[derive(Deserialize)]
struct Config {
    name: Option<String>,
}
