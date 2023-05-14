#![allow(unused)]
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{fs::OpenOptions, io::Write, io::Error, string::String};
use error_chain::error_chain;
use rand;
use sqlx::postgres::{PgPoolOptions, PgRow, PgConnectOptions};
use sqlx::{FromRow, Row, ConnectOptions};
use actix_web::{rt,get,post, http, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
use actix_cors::Cors;
use serde_json::json;

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

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct User
{
    id: i32,
    username: String,
    password: String
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}

// #[get("/")]
// async fn send_data() -> String
// {
//     let mut stream = sqlx::query_as::<_, User>("select id, username, password from data")
//         .fetch_all(&mut conn)
//         .await.expect("Failed to make a querry");
//     for i in 0..stream.len()
//     {
//         let mut dec_username = key.decrypt_base64_to_string(&(stream[i].username));
//         let mut err_status: bool = false;
//         match dec_username {
//             Ok(_) => (),
//             Err(_) => {
//             println!("Incorrect master key!");
//             err_status = true;
//             }
//         };
//         if (!err_status)
//         {
//             let dec_password =  key.decrypt_base64_to_string(&(stream[i].password)).unwrap();
//         }
//     }
//     dec_password
// }

#[post("/")]
async fn get_data(master_key: String) -> String
{
    //
    // let mut conn = PgConnectOptions::new()
    //     .host("localhost")
    //     .port(5432)
    //     .username("postgres")
    //     .password("api248")
    //     .connect().await.expect("Failed to connect");
    // let mut id = 0;
    // let key = new_magic_crypt!(master_key, 256);
    // let mut stream = sqlx::query_as::<_, User>("select id, username, password from data")
    //     .fetch_all(&mut conn)
    //     .await.expect("Failed to make a querry");
    // let mut buf = String::new();
    // for i in 0..stream.len()
    // {
    //     let mut dec_username = key.decrypt_base64_to_string(&(stream[i].username));
    //     let mut err_status: bool = false;
    //     match dec_username {
    //         Ok(_) => (),
    //         Err(_) => {
    //         buf.push_str("Incorrect master key!");
    //         buf.push('\n');
    //         err_status = true;
    //         }
    //     };
    //     if (!err_status)
    //     {
    //         let dec_password =  key.decrypt_base64_to_string(&(stream[i].password)).unwrap();
    //         buf.push_str(&dec_username.unwrap());
    //         buf.push('\t');
    //         buf.push_str(&dec_password);
    //         buf.push('\n');
    //     }
    // }
    // buf
    let mut conn = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("postgres")
        .password("api248")
        .connect().await.expect("Failed to connect");
    let mut id = 0;
    let key = new_magic_crypt!(master_key, 256);
    loop {
        let mode =
            input!("Press '1' - to show list. Press '2' to add new item. Press '3' to delete. Press 'q' to quit.")
                .to_lowercase();

        if mode == "q" {
            break;
        } else if mode == "1" {
            let mut stream = sqlx::query_as::<_, User>("select id, username, password from data")
                .fetch_all(&mut conn)
                .await.expect("Failed to make a querry");
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
            .await
            .expect("Failed to querry");

            let row: (i32,) = sqlx::query_as("insert into data values ($1, $2, $3) returning id")
                .bind(id)
                .bind(enc_account_name)
                .bind(enc_password)
                .fetch_one(&mut conn)
                .await
                .expect("Failed to connect");
        }
        else if mode == "3"
        {
                        let username = input!("Account Name to delete:");
                        let enc_username = key.encrypt_str_to_base64(username);
                        let row: (i32,) = sqlx::query_as("DELETE FROM data WHERE username = ($1) returning id")
                            .bind(enc_username)
                            .fetch_one(&mut conn)
                            .await
                            .expect("Failed to connect");
        }
        else {
            println!("Invalid command.Please try again.");
        }
    }
    let status = "Ok".to_string();
    status
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    HttpServer::new(|| {
        let cors = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);
        App::new()
            .wrap(cors)
            .service(get_data)
            .route("/", web::post().to(index))
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await;

    Ok(())
}
