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

use anyhow::Ok;
use sqlx::PgPool;

pub mod admin;

cfg_if::cfg_if! {
    if #[cfg(feature = "sqlite")] {
        pub const SQLX_DB: &str = "sqlite";
        const POSTGRES: bool = false;
    } else if #[cfg(feature = "postgres")] {
        const POSTGRES: bool = true;
        pub const SQLX_DB: &str = "postgres";
    } else {
        const POSTGRES: bool = false;
        pub const SQLX_DB: &str = "No Database Feature set in your Cargo.toml, should be sqlite or postgres";
    }
}

//pub async fn initialize(pool: &Pool<sqlx::AnyPool>) -> anyhow::Result<()> {
pub async fn initialize(_pool: &PgPool) -> anyhow::Result<()> {
    assert!(SQLX_DB.ne("No Database Feature set in your Cargo.toml, should be sqlite or postgres"));
    println!("Initializing axum-tenancy for DB: {}", SQLX_DB);
    //admin_postgres::user_postgres::insert();
/*    
    //cfg_if::cfg_if! {
    //    if #[cfg(feature = "sqlite")] {
    //        sqlx::migrate!("migrations/sqlite").run(pool).await?;
    //    } else if #[cfg(feature = "postgres")] {
    //        sqlx::migrate!("migrations/postgres").run(pool).await?;
    //    } else {
    //        assert_eq!("No Db feature", "In Cargo.toml for axum-tenancy");
    //        // never yet here but ensure that pool is used
    //        sqlx::migrate!("migrations/sqlite").run(pool).await?;
    //    } 
    //}
*/    

 /*
    * println!("pre migrate");
    sqlx::migrate!("migrations/postgres").run(pool).await?;
    println!("migrated");

    let tx: &mut sqlx::Transaction<'_, sqlx::Postgres> = &mut pool.begin().await?;
    println!("tx");

    let ru = insert(tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
    println!("ins");

    let uid = ru.unwrap_or_default();
    println!("uuid: {}", uid);
    let ur = load_by_id(tx, uid).await;
    let u = ur.unwrap_or_default();
    println!("u.user_name: {}", u.user_name);
    eprintln!("sort:{}",UserSort::UserName.as_str());
    */
    Ok(())
}



