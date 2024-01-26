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

use anyhow::{anyhow, Result, Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub display_name: String,
    pub is_admin: bool,
    pub email: String,
    pub mobile_phone: String,
}

impl Default for User {
    fn default() -> User {
        User {
            user_id: Uuid::new_v4(),
            user_name: "".to_string(),
            display_name: "".to_string(),
            is_admin: true,
            email: "".to_string(),
            mobile_phone: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Users {
    users: Vec<User>,
}
/*
pub fn create_routes() -> Router<AppState<'static>> {
    Router::new()
        .route("/", get(list))
        //.route("/:id", get(display))
        //.route("/add", get(add))
        //.route("/edit/:id", get(edit))
        //.route("/insert", post(insert))
        //.route("/delete/:id", post(delete))
    //.route("/password/:id", get(get_password))
    //.route("/password/:id", post(set_password))

}

pub async fn list(
    mut tx: Tx,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let appusers = list_all(tx).await;

    render_into_response(state, "admin/user/list.html", &page)
}
*/


#[allow(dead_code)]
pub async fn insert(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
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
                return Ok(user_id);
            } else {
                return Err(anyhow!("Insert did not return 1 row affected:{}",qr.rows_affected()));
            }
        }
        Err(e) => Err(e.into()),
    }
}

#[allow(dead_code)]
pub async fn load_by_id(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Uuid)
-> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT user_id, user_name, display_name, is_admin, email, mobile_phone from "user" where user_id = $1"#,
        &user_id
    )
    .fetch_one(&mut **tx)
    .await
}

#[allow(dead_code)]
pub async fn update(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: &Uuid,
    user_name: &str,
    display_name: &str,
    is_admin: bool,
    email: &str,
    mobile_phone: &str,
) -> Result<u64, Error> {
    let hash_password = "".to_string();

    let r = sqlx::query!(
        r#"
        UPDATE "user" 
            SET user_name = $2,
                hash_password = $3, 
                display_name = $4, 
                is_admin = $5, 
                email = $6, 
                mobile_phone = $7 
            WHERE
                user_id = $1
        "#,
        user_id,
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
        Ok(qr) => return Ok(qr.rows_affected()),
        Err(e) => Err(e.into()),
    }
}


#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use super::*;

    #[sqlx::test(migrations = "migrations/postgres")]
    async fn insert_user_no_dup_user_name(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let ru = insert(&mut tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&ru.is_ok(), &true);
        assert_eq!(insert(&mut tx, "Dave", "not Dave Warnock", true, "dwarnock@test.com", "01234567891").await.is_err(), true);

        Ok(())
    }

    #[sqlx::test(migrations = "migrations/postgres")]
    async fn insert_user_no_dup_display_name(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let ru = insert(&mut tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&ru.is_ok(), &true);
        assert_eq!(insert(&mut tx, "NotDave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await.is_err(), true);

        Ok(())
    }

    #[sqlx::test(migrations = "migrations/postgres")]
    async fn insert_then_check_load_user(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let ru = insert(&mut tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&ru.is_ok(), &true);

        let uid = ru.unwrap_or_default();
        assert_ne!(&uid.to_string(), ""); // uuid must not be empty

        let ur = load_by_id(&mut tx, uid).await;
        assert_eq!(&ur.is_ok(), &true);  // load_by_id reult is ok
        
        let u = ur.unwrap_or_default();
        assert_eq!(&u.user_id, &uid);
        assert_eq!(&u.user_name.to_string(), &"Dave");
        assert_eq!(&u.display_name.to_string(), &"Dave Warnock");
        assert_eq!(&u.is_admin, &true);
        assert_eq!(&u.email.to_string(), &"dwarnock@test.com");
        assert_eq!(&u.mobile_phone.to_string(), &"01234567891");

        Ok(())
    }
}
