#[macro_use]
extern crate log;

use std::env;
use std::fs::File;
use std::io::{copy, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

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
    let image_base_path = PathBuf::from(env::var_os("TMPDIR").unwrap());

    for target in targets {
        info!("Building data for {} - {}", target.hostname, target.github);

        let filepath = image_base_path
            .clone()
            .join(format!("{}.png", target.github));

        let mut image_file = File::create(filepath.clone()).unwrap();
        let mut request =
            reqwest::get(&format!("https://github.com/{}.png", target.github)).unwrap();

        copy(&mut request, &mut image_file).unwrap();

        let image = touchbar.create_image_from_path(filepath.to_str().unwrap());
        let hostname = target.hostname.clone();
        let target_button_id = touchbar.create_button(
            Some(&image),
            None,
            Box::new(move |_| {
                info!("Button clicked - hostname: {}", hostname);
                let child = Command::new("osascript")
                    .stdin(Stdio::piped())
                    .spawn()
                    .unwrap();

                info!("Spawned osascript");

                let mut stdin = child.stdin.unwrap();
                let script =
                    format!(
                        r#"tell application "Screen Sharing"
                            activate
                            tell application "System Events"
                                keystroke "{}.local"
                                keystroke return
                                delay 1
                                tell application "System Events"
                                    click (radio button 1 of radio group 1 of window 1) of application process "Screen Sharing"
                                    keystroke return
                                end tell
                            end tell
                        end tell"#, hostname);

                info!("Sending script");

                stdin.write(&script.as_bytes()).unwrap();

                info!("Done");
            }),
        );
        touchbar.update_button_width(&target_button_id, 50);

        button_ids.push(target_button_id);
    }

    info!("Done building data for buttons");

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
