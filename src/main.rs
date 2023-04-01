use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{fs::OpenOptions, io::Write, io::copy, fs::File};
use error_chain::error_chain;
use tempfile::Builder;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

macro_rules! input {
    ($prompt:expr) => {{
        println!("{}", $prompt);
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to get input in macros 'input'.");
        buf.trim().to_string()
    }};
}

#[tokio::main]
async fn main() -> Result<()> {
    let master_key = input!("Enter the key");
    loop {
        let mode =
            input!("Press '1' - to show list. Press '2' to add new item. Press 'q' to quit.")
                .to_lowercase();

        if mode == "q" {
            break;
        } else if mode == "1" {
            let tmp_dir = Builder::new().prefix("data").tempdir()?;
            let target = "https://d.zaix.ru/yGVA.txt";
            let response = reqwest::get(target).await?;

            let mut dest = {
                let fname = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");

                println!("file to download: '{}'", fname);
                let fname = tmp_dir.path().join(fname);
                println!("will be located under: '{:?}'", fname);
                File::create(fname)?
            };
            let content =  response.text().await?;
            copy(&mut content.as_bytes(), &mut dest)?;
            show(&master_key, content);
        } else if mode == "2" {
            add(&master_key);
        } else {
            println!("Invalid command.Please try again.");
        }
    }
    Ok(())
}

fn add(master_key: &str) {
    let account_name = input!("Account Name: ");
    let password = input!("Password: ");
    let magic_crypt = new_magic_crypt!(master_key, 256);
    let base_64_pwd = magic_crypt.encrypt_bytes_to_base64(&password);

    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.txt")
        .expect("Failed to open file to write data");
    f.write_all(&format!("{account_name}|{base_64_pwd}\n").as_bytes())
        .expect("Failed to save data");
}

fn show(master_key: &str, content: String) {
    let data = content;
    let magic_crypt = new_magic_crypt!(master_key, 256);
    for line in data.lines() {
        let (account_name, password) = line.split_once("|").expect("Invalid data was loaded");
        let pwd = magic_crypt.decrypt_base64_to_string(password);
        match pwd {
            Ok(p) => println!("{account_name} {p}"),
            Err(_) => {
                println!("Incorrect key!");
            }
        };
    }
}
