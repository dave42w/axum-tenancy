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

pub enum UserSort {
    UserName,
    DisplayName,
}

impl UserSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserSort::UserName => "user_name",
            UserSort::DisplayName => "display_name"
        }
    }
}/*
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

// currently if I pass the field name for order by using bind it is ignored TODO!
#[allow(dead_code)]
pub async fn load_all_sorted(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sort: UserSort)
-> Result<Vec<User>, sqlx::Error> {
   match sort {
        UserSort::UserName => {
            sqlx::query_as!(
                User,
                r#"SELECT user_id, user_name, display_name, is_admin, email, mobile_phone FROM "user" ORDER BY user_name ASC"#
            )
            .fetch_all(&mut **tx)
            .await
        }
        UserSort::DisplayName => {
            sqlx::query_as!(
                User,
                r#"SELECT user_id, user_name, display_name, is_admin, email, mobile_phone FROM "user" ORDER BY display_name ASC"#
            )
            .fetch_all(&mut **tx)
            .await
        }
    }
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
        let user_result = insert(&mut tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&user_result.is_ok(), &true);
        assert_eq!(insert(&mut tx, "Dave", "not Dave Warnock", true, "dwarnock@test.com", "01234567891").await.is_err(), true);

        Ok(())
    }

    #[sqlx::test(migrations = "migrations/postgres")]
    async fn insert_user_no_dup_display_name(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let user_result = insert(&mut tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&user_result.is_ok(), &true);
        assert_eq!(insert(&mut tx, "NotDave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await.is_err(), true);

        Ok(())
    }

    #[sqlx::test(migrations = "migrations/postgres")]
    async fn insert_then_check_load_user(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let user_result = insert(&mut tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&user_result.is_ok(), &true);

        let inserted_uuid = user_result.unwrap_or_default();
        assert_ne!(&inserted_uuid.to_string(), ""); // uuid must not be empty

        let load_result = load_by_id(&mut tx, inserted_uuid).await;
        assert_eq!(&load_result.is_ok(), &true);  // load_by_id reult is ok
        
        let loaded_user = load_result.unwrap_or_default();
        assert_eq!(&loaded_user.user_id, &inserted_uuid);
        assert_eq!(&loaded_user.user_name.to_string(), &"Dave");
        assert_eq!(&loaded_user.display_name.to_string(), &"Dave Warnock");
        assert_eq!(&loaded_user.is_admin, &true);
        assert_eq!(&loaded_user.email.to_string(), &"dwarnock@test.com");
        assert_eq!(&loaded_user.mobile_phone.to_string(), &"01234567891");

        Ok(())
    }

    #[sqlx::test(migrations = "migrations/postgres")]
    async fn insert_update_then_check(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let insert_result = insert(&mut tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&insert_result.is_ok(), &true);

        let inserted_uuid = insert_result.unwrap_or_default();
        assert_ne!(&inserted_uuid.to_string(), ""); // uuid must not be empty

        let update_result = update(&mut tx, &inserted_uuid, "not Dave", "not Dave Warnock", false, "not dwarnock@test.com", "6601234567891").await;
        assert_eq!(&update_result.is_ok(), &true);
        let rows_affected = update_result.unwrap();
        assert_eq!(rows_affected, 1);

        let load_result = load_by_id(&mut tx, inserted_uuid).await;
        assert_eq!(&load_result.is_ok(), &true);  // load_by_id reult is ok
        
        let loaded_user = load_result.unwrap_or_default();
        assert_eq!(&loaded_user.user_id, &inserted_uuid);
        assert_eq!(&loaded_user.user_name.to_string(), &"not Dave");
        assert_eq!(&loaded_user.display_name.to_string(), &"not Dave Warnock");
        assert_eq!(&loaded_user.is_admin, &false);
        assert_eq!(&loaded_user.email.to_string(), &"not dwarnock@test.com");
        assert_eq!(&loaded_user.mobile_phone.to_string(), &"6601234567891");

        Ok(())
    }

    #[sqlx::test(migrations = "migrations/postgres")]
    async fn insert_two_check_load_all(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let user_result1 = insert(&mut tx, "zDave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&user_result1.is_ok(), &true);

        let inserted_uuid1 = user_result1.unwrap_or_default();
        assert_ne!(&inserted_uuid1.to_string(), ""); // uuid must not be empty

        let user_result2 = insert(&mut tx, "Dave2", "Dave Warnock2", false, "dwarnock@test.com2", "012345678912").await;
        assert_eq!(&user_result2.is_ok(), &true);

        let inserted_uuid2 = user_result2.unwrap_or_default();
        assert_ne!(&inserted_uuid2.to_string(), ""); // uuid must not be empty

        let load_result = load_all_sorted(&mut tx, UserSort::UserName).await;
        assert_eq!(&load_result.is_ok(), &true);  // load_by_id reult is ok
        let vec_users = &load_result.unwrap();
        assert_eq!(&vec_users.len(), &2usize);  // load_by_id reult is ok
        
        let loaded_user2 = &vec_users[0];
        assert_eq!(&loaded_user2.user_id, &inserted_uuid2);
        assert_eq!(&loaded_user2.user_name.to_string(), &"Dave2");
        assert_eq!(&loaded_user2.display_name.to_string(), &"Dave Warnock2");
        assert_eq!(&loaded_user2.is_admin, &false);
        assert_eq!(&loaded_user2.email.to_string(), &"dwarnock@test.com2");
        assert_eq!(&loaded_user2.mobile_phone.to_string(), &"012345678912");

        let loaded_user1 = &vec_users[1];
        assert_eq!(&loaded_user1.user_id, &inserted_uuid1);
        assert_eq!(&loaded_user1.user_name.to_string(), &"zDave");
        assert_eq!(&loaded_user1.display_name.to_string(), &"Dave Warnock");
        assert_eq!(&loaded_user1.is_admin, &true);
        assert_eq!(&loaded_user1.email.to_string(), &"dwarnock@test.com");
        assert_eq!(&loaded_user1.mobile_phone.to_string(), &"01234567891");

        Ok(())
    }
}
