/*
# MIT License
#
# Copyright (c) 2024 Dave Warnock
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
*/

use std::env;

use axum_tenancy;
use dotenvy::dotenv;
use sqlx::{any::install_default_drivers, postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv().expect(".env file not found");
    let _server_uri: String = env::var("SERVER_URI").expect(".env missing SERVER");
    let database_url: String = env::var("DATABASE_PARTIAL_URL").expect(".env missing DATABASE_URL");
    let database_pw: String = env::var("DATABASE_PW").expect(".env missing DATABASE_PW");
    //install_default_drivers();
    let pool_options = PgPoolOptions::new();
    let uri = format!("{}={}", database_url, database_pw);
    println!("uri:{}", &uri);
    //let pool: Pool<sqlx::PgPool> = pool_options.connect(&uri).await.unwrap();
    let pool = pool_options.connect(&uri).await.unwrap();
    println!("pool created");
    let _ = axum_tenancy::initialize(&pool).await;
    println!("init donei");
}
