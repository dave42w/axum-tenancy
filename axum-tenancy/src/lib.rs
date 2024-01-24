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

//use sqlx::{self, Postgres};
use anyhow::Context;
use axum_sqlx_tx;

mod admin;

type Tx = axum_sqlx_tx::Tx<sqlx::Any>;
type DbPool = sqlx::AnyPool;

cfg_if::cfg_if! {
    if #[cfg(feature = "sqlite")] {
        //type Tx = axum_sqlx_tx::Tx<sqlx::Sqlite>;
        //type DbPool = sqlx::SqlitePool;
        pub const SQLX_DB: &str = "sqlite";
    } else if #[cfg(feature = "postgres")] {
        //type Tx = axum_sqlx_tx::Tx<sqlx::Postgres>;
        //type DbPool = sqlx::PgPool;
        pub const SQLX_DB: &str = "postgres";
    } else {
        //type Tx = axum_sqlx_tx::Tx<sqlx::Sqlite>;
        //type DbPool = sqlx::SqlitePool;
        pub const SQLX_DB: &str = "No Database Feature set in your Cargo.toml, should be sqlite or postgres";
    }
}

pub async fn initialize(pool: &DbPool) -> anyhow::Result<()> {
    assert!(SQLX_DB.ne("No Database Feature set in your Cargo.toml, should be sqlite or postgres"));
    println!("Initializing axum-tenancy for DB: {}", SQLX_DB);
    cfg_if::cfg_if! {
        if #[cfg(feature = "sqlite")] {
            sqlx::migrate!("migrations/sqlite").run(pool).await?;
        } else if #[cfg(feature = "postgres")] {
            sqlx::migrate!("migrations/postgres").run(pool).await?;
        } else {
            assert!("Migration not possible, no DB feature");
        } 
    }
    Ok(())
}



