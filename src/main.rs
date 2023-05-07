#![allow(unused)]
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{fs::OpenOptions, io::Write, io::Error, string::String};
use error_chain::error_chain;
use rand;
use sqlx::postgres::{PgPoolOptions, PgRow, PgConnectOptions};
use sqlx::{FromRow, Row, ConnectOptions};
use aes_gcm::{
        aead::{Aead, KeyInit, OsRng},
        Aes256Gcm, Nonce
};

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct User
{
    id: i32,
    username: Vec<i32>,
    password: Vec<i32>
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
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce");
    let master_key = input!("Enter key: -->");
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
                let mut v1: Vec<u8> = Vec::with_capacity(stream[i].username.len());
                let mut v2: Vec<u8> = Vec::with_capacity(stream[i].password.len());
                let username_bytes: &[u8] = unsafe {
                    std::slice::from_raw_parts(
                        stream[i].username.as_ptr() as *const u8,
                        stream[i].username.len() * std::mem::size_of::<i32>(),
                    )
                };
                let password_bytes: &[u8] = unsafe {
                    std::slice::from_raw_parts(
                        stream[i].password.as_ptr() as *const u8,
                        stream[i].password.len() * std::mem::size_of::<i32>(),
                    )
                };
                for j in 0..username_bytes.len()
                {
                    if (username_bytes[j]!=0)
                    {
                        v1.push(username_bytes[j]);
                    }
                }
                for j in 0..password_bytes.len()
                {
                    if (password_bytes[j]!=0)
                    {
                        v2.push(password_bytes[j]);
                    }
                }

                let dec_username = cipher.decrypt(nonce, v1.as_ref()).expect("Failed to decrypt");
                let dec_password = cipher.decrypt(nonce, v2.as_ref()).expect("Failed to decrypt");
                let str_1 = String::from_utf8(dec_username);
                let str_2 = String::from_utf8(dec_password);
                println!("{:?}", str_1);
                println!("{:?}", str_2);
            }

        } else if mode == "2" {
            let account_name = input!("Account Name: ");
            let password = input!("Password: ");
            id += 1;
            let password_to_bytes: &[u8] = password.as_bytes();
            let account_name_to_bytes: &[u8] = account_name.as_bytes();
            let enc_account_name = cipher.encrypt(nonce, account_name_to_bytes.as_ref()).expect("Failed to encrypt");
            let enc_password = cipher.encrypt(nonce, password_to_bytes.as_ref()).expect("Failed to encrypt");
            sqlx::query(
            		r#"
            CREATE TABLE IF NOT EXISTS data (
              id INT,
              username integer[],
              password integer[]
            );"#,
            )
            .execute(&mut conn)
            .await?;

            let capacity = enc_password.len();
            let mut v: Vec<i32> = Vec::with_capacity(capacity);
            for i in 0..capacity {
                let x = i32::from(enc_password[i]);
                v.push(x);
            }
            let capacity_2 = enc_account_name.len();
            let mut v_2: Vec<i32> = Vec::with_capacity(capacity_2);
            for i in 0..capacity_2 {
                let x_2 = i32::from(enc_account_name[i]);
                v_2.push(x_2);
            }

            let row: (i32,) = sqlx::query_as("insert into data values ($1, $2, $3) returning id")
                .bind(id)
                .bind(v_2)
                .bind(v)
                .fetch_one(&mut conn)
                .await?;
        } else {
            println!("Invalid command.Please try again.");
        }
    }
    Ok(())
}

// fn encrypt(cipher:Aes256Gcm, nonce:Nonce)
