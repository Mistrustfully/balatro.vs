use anyhow::Result;
use once_cell::sync::Lazy;
use std::{path::PathBuf, process::Command};
use tokio::fs;

/// The directory of our mod files, relative to the project's root
const LUA_DIR: &'static str = "./lua";
static BASE_DIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("AppData/Roaming/Balatro/Mods/balatro.vs"));

fn mod_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    return dirs::home_dir().and_then(|mut dir| {
        dir.push(BASE_DIR.clone());
        Some(dir)
    });

    #[cfg(target_os = "linux")]
    return dirs::home_dir().and_then(|mut dir| {
        dir.push(".steam/steam/steamapps/compatdata/2379780/pfx/drive_c/users/steamuser/");
        dir.push(BASE_DIR.clone());
        Some(dir)
    });

    #[cfg(target_os = "macos")]
    {
        todo!("Mac OS implementation not done");
    }
}

/// Copies the `lua` directory into the Balatro mod folder
pub async fn build() -> Result<()> {
    let mod_dir = mod_dir().ok_or(anyhow::Error::msg("Failed to get home directory"))?;

    // If the our mod folder already exists, delete it
    match fs::metadata(&mod_dir).await {
        Ok(file) => {
            if file.is_dir() {
                fs::remove_dir_all(&mod_dir).await?;
            } else if file.is_file() {
                fs::remove_file(&mod_dir).await?;
            }
        }
        Err(_) => {}
    }

    // Create new directoru
    fs::create_dir_all(&mod_dir).await?;

    // Copy all files from `./lua into our new directory`
    let mut lua_folder = fs::read_dir(LUA_DIR).await?;
    while let Ok(Some(entry)) = lua_folder.next_entry().await {
        let path = entry.path();
        let true_path = path.strip_prefix(LUA_DIR)?;
        let destination = &mod_dir.join(true_path);

        fs::copy(entry.path(), destination).await?;

        // Absolutely horrid code injection
        // Used to reduce code reuse as we cannot access `util.lua` from a Love2D thread
        if destination
            .file_name()
            .ok_or(anyhow::Error::msg("No file name for path"))?
            == "net.lua"
        {
            let original_file = fs::read_to_string(destination).await?;
            let util = fs::read_to_string("./lua/util.lua").await?;

            // Evaluates the util.lua module and puts the result into `package.loaded`
            // This way `require("util")` will work as expected.
            let injected_file = format!(
                r#"
                -- injected code
                do
                    local util = (function()
                        {util}
                    end)()

                    package.loaded["util"] = util
                end
                -- injected code end
                {original_file}
            "#
            );

            fs::write(destination, injected_file).await?;
        }
    }

    Ok(())
}

/// Builds the mod then launches steam.
pub async fn run() -> Result<()> {
    build().await?;

    Command::new("steam")
        .arg("steam://rungameid/2379780")
        .spawn()
        .expect("Failed to launch steam!");

    Ok(())
}
