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

use serde::{Deserialize, Serialize};
//use sqlx::Error;
//use super::super::Tx;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppUser {
    pub id: uuid::Uuid,
    pub user_name: String,
    pub display_name: String,
    pub is_admin: bool,
    pub email: String,
    pub mobile_phone: String,
}

impl Default for AppUser {
    fn default() -> AppUser {
        AppUser {
            id: uuid::Uuid::new_v4(),
            user_name: "".to_string(),
            display_name: "".to_string(),
            is_admin: true,
            email: "".to_string(),
            mobile_phone: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AppUsers {
    users: Vec<AppUser>,
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

/*
#[allow(dead_code)]
async fn insert(
    mut tx: Tx,
    user_name: &str,
    display_name: &str,
    is_admin: bool,
    email: &str,
    mobile_phone: &str,
) -> Result<(), Error> {
    let id = uuid::Uuid::new_v4();
    let hash_password = "".to_string();

    sqlx::query!(
        r#"
        INSERT INTO AppUser 
        (id, user_name, hash_password, display_name, is_admin, email, mobile_phone) 
        VALUES
        ($1, $2, $3, $4, $5, $6, $7)
        "#,
        id,
        user_name,
        hash_password,
        display_name,
        is_admin,
        email,
        mobile_phone
    )
    .execute(&mut tx)
    .await?;
    Ok(())
}
*/

/*
Waiting for how to get a axum-sqlx-tx transaction out of the pool
#[cfg(test)]
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::Transaction;

#[sqlx::test]
async fn check_insert_method(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
    let pgTx: Tx = pool.begin().await.unwrap().into();

    // test that you can insert a user but not insert a duplicxate user
    assert_eq!(insert(pgTx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await.is_ok(), true);
    assert_eq!(insert(pgTx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await.is_err(), true);
    Ok(())
}
*/