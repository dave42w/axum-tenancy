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

use axum_tenancy_core::ActiveDb;
use crate::ACTIVE_DB;
use anyhow::{Result, Error};
use uuid::Uuid;
use axum_tenancy_core::admin_core::user_core::{User, UserSort, SortDirection};

use axum_tenancy_postgres::admin_postgres::user_postgres;

pub async fn insert(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_name: &str,
    display_name: &str,
    is_admin: bool,
    email: &str,
    mobile_phone: &str,
) -> Result<uuid::Uuid, Error> {
    user_postgres::insert_postgres(tx, user_name, display_name, is_admin, email, mobile_phone).await
}

pub async fn load_by_id(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: Uuid)
-> Result<User, sqlx::Error> {
    user_postgres::load_by_id_postgres(tx, user_id).await
}
pub async fn load_all_sorted(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sort: UserSort,
    direction: SortDirection)
-> Result<Vec<User>, sqlx::Error> {
    user_postgres::load_all_sorted_postgres(tx, sort, direction).await
}

pub async fn update(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: &Uuid,
    user_name: &str,
    display_name: &str,
    is_admin: bool,
    email: &str,
    mobile_phone: &str,
) -> Result<u64, Error> {
    let u = User {
        user_id: *user_id,
        user_name: user_name.to_string(),
        display_name: display_name.to_string(),
        is_admin,
        email: email.to_string(),
        mobile_phone: mobile_phone.to_string(),
    };
    match ACTIVE_DB {
        ActiveDb::Postgres => {
            let r = user_postgres::update_postgres(tx, &u).await;
            match r {
                Ok(qr) => return Ok(qr.rows_affected()),
                Err(e) => Err(e.into()),
            }
        }
        ActiveDb::Sqlite => Err(Error::msg("No sqlite db code")),
        ActiveDb::Undefined => Err(Error::msg("Undefined db not supported")),    
    }
}


#[cfg(test)]
mod tests_tokio {
    use axum_tenancy_core::ActiveDb;

    cfg_if::cfg_if! {
        if #[cfg(feature = "sqlite")] {
            pub const ACTIVE_DB: ActiveDb = ActiveDb::Sqlite;
        } else if #[cfg(feature = "postgres")] {
            pub const ACTIVE_DB: ActiveDb = ActiveDb::Postgres;
        } else {
            pub const ACTIVE_DB: ActiveDb = ActiveDb::Undefined;
        }
    }
        
    #[tokio::test(flavor = "multi_thread")]
    async fn tokio_test() {
        /*
        cfg_if::cfg_if! {
            if #[cfg(feature = "postgres")] {
                assert!(true);
                const POSTGRES: bool = true;
            } else {
                assert!(false);
                const POSTGRES: bool = false;
            }
        }
        */
        assert_eq!(ACTIVE_DB, ActiveDb::Postgres);
    }
}

#[cfg(test)]
mod tests_postgres {
    use sqlx::PgPool;
    use super::*;

    #[sqlx::test(migrations = "../axum-tenancy-postgres/migrations")]
    async fn insert_user_no_dup_user_name(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let user_result = insert(&mut tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&user_result.is_ok(), &true);
        assert_eq!(insert(&mut tx, "Dave", "not Dave Warnock", true, "dwarnock@test.com", "01234567891").await.is_err(), true);

        Ok(())
    }

    #[sqlx::test(migrations = "../axum-tenancy-postgres/migrations")]
    async fn insert_user_no_dup_display_name(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let user_result = insert(&mut tx, "Dave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&user_result.is_ok(), &true);
        assert_eq!(insert(&mut tx, "NotDave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await.is_err(), true);

        Ok(())
    }

    #[sqlx::test(migrations = "../axum-tenancy-postgres/migrations")]
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

    #[sqlx::test(migrations = "../axum-tenancy-postgres/migrations")]
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

    #[sqlx::test(migrations = "../axum-tenancy-postgres/migrations")]
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

        let load_result = load_all_sorted(&mut tx, UserSort::UserName, SortDirection::Asc).await;
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

    #[sqlx::test(migrations = "../axum-tenancy-postgres/migrations")]
    async fn insert_two_check_sort_desc(pool: PgPool) -> sqlx::Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let user_result1 = insert(&mut tx, "zDave", "Dave Warnock", true, "dwarnock@test.com", "01234567891").await;
        assert_eq!(&user_result1.is_ok(), &true);

        let inserted_uuid1 = user_result1.unwrap_or_default();
        assert_ne!(&inserted_uuid1.to_string(), ""); // uuid must not be empty

        let user_result2 = insert(&mut tx, "Dave2", "Dave Warnock2", false, "dwarnock@test.com2", "012345678912").await;
        assert_eq!(&user_result2.is_ok(), &true);

        let inserted_uuid2 = user_result2.unwrap_or_default();
        assert_ne!(&inserted_uuid2.to_string(), ""); // uuid must not be empty

        let load_result = load_all_sorted(&mut tx, UserSort::DisplayName, SortDirection::Desc).await;
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
