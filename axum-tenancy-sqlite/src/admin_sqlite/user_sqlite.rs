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

use anyhow::{anyhow, Error, Result};
use axum_tenancy_core::admin_core::user_core::{SortDirection, User, UserSort};
use sqlx::sqlite;
use uuid::Uuid;

type DbTransaction<'c> = sqlx::Transaction<'c, sqlx::Sqlite>;

pub async fn insert(
    tx: &mut DbTransaction<'_>,
    //tx: &mut sqlx::Transaction<'_, Postgres>,
    user_name: &str,
    display_name: &str,
    is_admin: bool,
    email: &str,
    mobile_phone: &str,
) -> Result<uuid::Uuid, Error> {
    let user_id = Uuid::new_v4();
    let hash_password = "".to_string();
    let r = sqlx::query!(
        r#"
        INSERT INTO "user" 
        (user_id, user_name, hash_password, display_name, is_admin, email, mobile_phone) 
        VALUES
        ($1, $2, $3, $4, $5, $6, $7)
        "#,
        &user_id,
        user_name,
        hash_password,
        display_name,
        is_admin,
        email,
        mobile_phone
    )
    .execute(&mut **tx)
    .await;

    match r {
        Ok(qr) => {
            if qr.rows_affected() == 1 {
                Ok(user_id)
            } else {
                Err(anyhow!(
                    "Insert did not return 1 row affected:{}",
                    qr.rows_affected()
                ))
            }
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn load_by_id(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Uuid,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT user_id, user_name, display_name, is_admin, email, mobile_phone from "user" where user_id = $1"#,
        &user_id
    )
    .fetch_one(&mut **tx)
    .await
}

pub async fn load_all_sorted(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sort: UserSort,
    direction: SortDirection,
) -> Result<Vec<User>, sqlx::Error> {
    match direction {
        SortDirection::Asc => {
            sqlx::query_as!(
                User,
                r#"SELECT user_id, user_name, display_name, is_admin, email, mobile_phone FROM "user" ORDER BY 
                    CASE 
                          WHEN $1 = 'user_name' THEN user_name
                          WHEN $1 = 'display_name' THEN display_name
                    END ASC
                "#,
                sort.as_str()
            )
            .fetch_all(&mut **tx)
            .await
        }
        SortDirection::Desc => {
            sqlx::query_as!(
                User,
                r#"SELECT user_id, user_name, display_name, is_admin, email, mobile_phone FROM "user" ORDER BY 
                    CASE 
                          WHEN $1 = 'user_name' THEN user_name
                          WHEN $1 = 'display_name' THEN display_name
                    END DESC
                "#,
                sort.as_str()
            )
            .fetch_all(&mut **tx)
            .await
        }
    }
}

pub async fn update(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    u: &User,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE "user" 
            SET user_name = $2,
                display_name = $3, 
                is_admin = $4, 
                email = $5, 
                mobile_phone = $6 
            WHERE
                user_id = $1
        "#,
        u.user_id,
        u.user_name,
        u.display_name,
        u.is_admin,
        u.email,
        u.mobile_phone
    )
    .execute(&mut **tx)
    .await
}
