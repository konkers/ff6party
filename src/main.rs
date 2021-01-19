use anyhow::{anyhow, Result};
use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde_json::json;
use std::fs::File;
use usb2snes::Connection;

lazy_static! {
    static ref ACTORS: Vec<&'static str> = {
        let mut v = Vec::new();
        v.push("Terra");
        v.push("Locke");
        v.push("Cyan");
        v.push("Shadow");
        v.push("Edgar");
        v.push("Sabin");
        v.push("Celes");
        v.push("Strago");
        v.push("Relm");
        v.push("Setzer");
        v.push("Mog");
        v.push("Gau");
        v.push("Gogo");
        v.push("Umaru");
        v
    };
}

async fn run() -> Result<()> {
    let mut c = Connection::new("ws://localhost:8080")
        .await
        .map_err(|e| anyhow!("cannot connect: {}", e))?;
    let devs = c
        .get_device_list()
        .await
        .map_err(|e| anyhow!("cannot get device list: {}", e))?;
    let dev = devs[0].to_string();

    c.attach(&dev)
        .await
        .map_err(|e| anyhow!("cannot attach to {}: {}", &dev, e))?;

    let mut data = [0u8; 16];
    c.read_mem(0xf53000, &mut data)
        .await
        .map_err(|e| anyhow!("cannot read: {}", e))?;

    let mut party = [""; 4];
    for i in 0..ACTORS.len() {
        let party_index = data[i] as usize / 2;

        if party_index < party.len() {
            party[party_index] = ACTORS[i];
        } else if party_index != (0xff / 2) {
            return Ok(());
        }
    }

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "./index-template.html")?;
    let mut output_file = File::create("index.html")?;
    handlebars.render_to_write(
        "index",
        &json!({
        "party0": party[0],
        "party1": party[1],
        "party2": party[2],
        "party3": party[3],
        }),
        &mut output_file,
    )?;

    Ok(())
}

fn main() -> Result<()> {
    async_std::task::block_on(run())?;
    Ok(())
}
