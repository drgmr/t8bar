#[macro_use]
extern crate log;

use std::env;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use rubrail::ItemId;
use rubrail::TTouchbar;
use rubrail::Touchbar;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Target {
    hostname: String,
    github: String,
}

fn main() {
    fruitbasket::create_logger(".t8bar.log", fruitbasket::LogDir::Home, 5, 2).unwrap();

    let mut nsapp = fruitbasket::Trampoline::new("t8bar", "t8bar", "com.drgmr.t8bar")
        .version(env!("CARGO_PKG_VERSION"))
        .plist_key("LSBackgroundOnly", "1")
        .build(fruitbasket::InstallDir::Custom("target/".to_string()))
        .unwrap();

    nsapp.set_activation_policy(fruitbasket::ActivationPolicy::Prohibited);

    // let bar_rc = Rc::new(RefCell::new(Touchbar::alloc("t8bar")));

    let stopper = nsapp.stopper();
    let mut touchbar = Touchbar::alloc("t8bar");

    setup(&mut touchbar, stopper);

    nsapp
        .run(fruitbasket::RunPeriod::Forever)
        .expect("Failed to launch app");
}

fn setup(touchbar: &mut Touchbar, stopper: fruitbasket::FruitStopper) {
    let targets = targets_from_config();

    let mut root_bar = touchbar.create_bar();

    let mut button_ids = Vec::<ItemId>::new();

    for target in targets {
        info!("Building data for {} - {}", target.hostname, target.github);

        let hostname = target.hostname.clone();
        let target_button_id = touchbar.create_button(
            None,
            Some(&hostname.clone()),
            Box::new(move |_| {
                info!("Button clicked - hostname: {}", hostname);
            })
        );

        button_ids.push(target_button_id);
    }

    let quit_stopper = stopper.clone();
    let quit_button_id = touchbar.create_button(
        None,
        Some("Quit"),
        Box::new(move |_| {
            info!("Exit requested by user");
            quit_stopper.stop();
        }),
    );

    button_ids.push(quit_button_id);

    touchbar.add_items_to_bar(&mut root_bar, button_ids);

    touchbar.set_bar_as_root(root_bar);
}

fn targets_from_config() -> Vec<Target> {
    let home_path = env::var_os("HOME").unwrap();
    let config_path = PathBuf::from(home_path)
        .join(".config")
        .join("t8bar")
        .join("config.json");
    info!("Expected config path: {:?}", config_path);

    let mut file = File::open(config_path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let result: Vec<Target> = serde_json::from_str(&contents).unwrap();
    info!("Configuration acquired: {:#?}", result);

    result
}

