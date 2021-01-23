use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use log::error;
use rocket::{
    get,
    response::{content, Debug},
    routes, State,
};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::{path::Path, sync::Arc};
use tokio::{
    self,
    fs::File,
    io::AsyncReadExt,
    sync::Mutex,
    time::{sleep, Duration},
};
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

#[derive(Debug, Serialize, Clone, Copy)]
struct Actor {
    id: i32,
    name: &'static str,
}

async fn get_party(c: &mut Connection) -> Result<[Actor; 4]> {
    let mut data = [0u8; 1];
    c.read_mem(0xf51a6d, &mut data)
        .await
        .map_err(|e| anyhow!("cannot read: {}", e))?;
    let cur_party = data[0];

    let mut data = [0u8; 16];
    c.read_mem(0xf51850, &mut data)
        .await
        .map_err(|e| anyhow!("cannot read: {}", e))?;

    let mut party = [
        Actor { id: -1, name: "" },
        Actor { id: -2, name: "" },
        Actor { id: -3, name: "" },
        Actor { id: -4, name: "" },
    ];
    for i in 0..ACTORS.len() {
        let party_num = data[i] & 0x7;
        let battle_order = (data[i] >> 3) & 0x3;

        if party_num == cur_party {
            party[battle_order as usize] = Actor {
                id: i as i32,
                name: ACTORS[i],
            };
        }
    }
    Ok(party)
}

async fn static_file(path: impl AsRef<Path>) -> std::result::Result<String, Debug<anyhow::Error>> {
    let path = path.as_ref();
    let mut file = File::open(path)
        .await
        .map_err(|e| anyhow!("can't open {}: {}", path.to_string_lossy(), e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .map_err(|e| anyhow!("can't read {}: {}", path.to_string_lossy(), e))?;

    Ok(contents)
}

#[get("/party")]
async fn party(
    c: State<'_, Arc<Mutex<Connection>>>,
) -> std::result::Result<Json<[Actor; 4]>, Debug<anyhow::Error>> {
    let mut lock = c.lock().await;
    let party = get_party(&mut lock).await?;

    Ok(Json(party))
}

#[get("/")]
async fn index() -> std::result::Result<content::Html<String>, Debug<anyhow::Error>> {
    let content = static_file("static/index.html").await?;
    Ok(content::Html(content))
}

#[get("/style.css")]
async fn style() -> std::result::Result<content::Css<String>, Debug<anyhow::Error>> {
    let content = static_file("static/style.css").await?;
    Ok(content::Css(content))
}

#[get("/index.js")]
async fn js() -> std::result::Result<content::JavaScript<String>, Debug<anyhow::Error>> {
    let content = static_file("static/index.js").await?;
    Ok(content::JavaScript(content))
}

async fn run() -> Result<()> {
    let mut c = Connection::new("ws://localhost:8080")
        .await
        .map_err(|e| anyhow!("cannot connect to usb2snes service: {}", e))?;
    let devs = c
        .get_device_list()
        .await
        .map_err(|e| anyhow!("cannot get usb2snes device list: {}", e))?;

    if devs.len() == 0 {
        return Err(anyhow!("No usb2snes devices connected."));
    }
    let dev = devs[0].to_string();

    c.attach(&dev)
        .await
        .map_err(|e| anyhow!("cannot attach to {}: {}", &dev, e))?;

    let web = rocket::ignite()
        .manage(Arc::new(Mutex::new(c)))
        .mount("/", routes![index, style, js, party])
        .launch();

    println!("tracker started at http://127.0.0.1:8000");

    tokio::select! {
        _ = web => {
        }
    }
    Ok(())
}
#[tokio::main]
pub async fn main() -> Result<()> {
    env_logger::init();

    if let Err(e) = run().await {
        error!("{}", e);
        error!("Waiting 5 seconds before closing");
        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}
