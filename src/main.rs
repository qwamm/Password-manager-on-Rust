#![allow(unused)]
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{fs::OpenOptions, io::Write, io::Error, string::String};
use error_chain::error_chain;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand;
use sqlx::postgres::{PgPoolOptions, PgRow, PgConnectOptions};
use sqlx::{FromRow, Row, ConnectOptions};

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
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    // let file = OpenOptions::new()
    //             .read(true)
    //             .write(true)
    //             .create(true)
    //             .open("foo.txt");
    // file?.write_all(private_key);
    //let master_key = input!("Enter key: -->");
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
                println!("{:?}",v1);
                let dec_username = private_key.decrypt(Pkcs1v15Encrypt, &v1).expect("failed to decrypt");
                let dec_password = private_key.decrypt(Pkcs1v15Encrypt, &v2).expect("failed to decrypt");
                let str_1 = String::from_utf8(dec_username);
                let str_2 = String::from_utf8(dec_password);
                println!("username: {:?}", str_1);
                println!("password: {:?}", str_2);
            }
        } else if mode == "2" {
            let account_name = input!("Account Name: ");
            let password = input!("Password: ");
            id += 1;
            // let password_vec_bytes = password.into_bytes();
            // let account_name_vec_bytes = account_name.into_bytes();
            let password_to_bytes = password.as_bytes();
            let account_name_to_bytes = account_name.as_bytes();
            //let s = String::from_utf8(account_name).unwrap();
             let enc_account_name = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &account_name_to_bytes[..]).expect("failed to encrypt");
             let enc_password = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &password_to_bytes[..]).expect("failed to encrypt");
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
