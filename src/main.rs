#![allow(unused)]
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{fs::OpenOptions, io::Write, io::Error, string::String};
use error_chain::error_chain;
use rand;
use sqlx::postgres::{PgPoolOptions, PgRow, PgConnectOptions};
use sqlx::{FromRow, Row, ConnectOptions};

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct User
{
    id: i32,
    username: String,
    password: String
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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("api248")
        .connect().await?;
    let mut id = 0;
    let master_key = input!("Enter key: -->");
    let key = new_magic_crypt!(master_key, 256);

    loop {
        let mode =
            input!("Press '1' - to show list. Press '2' to add new item. Press 'q' to quit.")
                .to_lowercase();

        if mode == "q" {
            break;
        } else if mode == "1" {
            let mut stream = sqlx::query_as::<_, User>("select id, username, password from data")
                .fetch_all(&mut conn)
                .await?;
            for i in 0..stream.len()
            {
                let mut dec_username = key.decrypt_base64_to_string(&(stream[i].username));
                let mut err_status: bool = false;
                match dec_username {
                    Ok(_) => (),
                    Err(_) => {
                    println!("Incorrect master key!");
                    err_status = true;
                    }
                };
                if (!err_status)
                {
                    let dec_password =  key.decrypt_base64_to_string(&(stream[i].password)).unwrap();
                    println!("{}", dec_username.unwrap());
                    println!("{}", dec_password);
                }
            }

        } else if mode == "2" {
            let account_name = input!("Account Name: ");
            let password = input!("Password: ");
            id += 1;
            let enc_account_name = key.encrypt_str_to_base64(account_name);
            let enc_password = key.encrypt_str_to_base64(password);
            sqlx::query(
            		r#"
            CREATE TABLE IF NOT EXISTS data (
              id INT,
              username text,
              password text
            );"#,
            )
            .execute(&mut conn)
            .await?;

            let row: (i32,) = sqlx::query_as("insert into data values ($1, $2, $3) returning id")
                .bind(id)
                .bind(enc_account_name)
                .bind(enc_password)
                .fetch_one(&mut conn)
                .await?;
        } else {
            println!("Invalid command.Please try again.");
        }
    }
    Ok(())
}
