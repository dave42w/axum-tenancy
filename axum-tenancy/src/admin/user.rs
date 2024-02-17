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

use anyhow::{Error, Result};
use axum_tenancy_core::admin_core::user_core::{SortDirection, User, UserSort};
use uuid::Uuid;

#[cfg(not(any(feature = "sqlite", feature = "postgres")))]
compile_error!("Either feature \"sqlite\" or \"postgres\" must be enabled for this crate.");

#[cfg(feature = "postgres")]
use axum_tenancy_postgres::admin_postgres::user_postgres as user_db;
#[cfg(feature = "sqlite")]
use axum_tenancy_sqlite::admin_sqlite::user_sqlite as user_db;


#[cfg(feature = "postgres")]
type DbTransaction<'c> = sqlx::Transaction<'c, sqlx::Postgres>;
#[cfg(feature = "sqlite")]
type DbTransaction<'c> = sqlx::Transaction<'c, sqlx::Sqlite>;

pub async fn insert(
    tx: &mut DbTransaction<'_>,
    user_name: &str,
    display_name: &str,
    is_admin: bool,
    email: &str,
    mobile_phone: &str,
) -> Result<uuid::Uuid, Error> {
    user_db::insert(tx, user_name, display_name, is_admin, email, mobile_phone).await
}

pub async fn load_by_id(tx: &mut DbTransaction<'_>, user_id: Uuid) -> Result<User, sqlx::Error> {
    user_db::load_by_id(tx, user_id).await
}
pub async fn load_all_sorted(
    tx: &mut DbTransaction<'_>,
    sort: UserSort,
    direction: SortDirection,
) -> Result<Vec<User>, sqlx::Error> {
    user_db::load_all_sorted(tx, sort, direction).await
}

pub async fn update(
    tx: &mut DbTransaction<'_>,
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
    let r = user_db::update(tx, &u).await;
    match r {
        Ok(qr) => Ok(qr.rows_affected()),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests_tokio {
    use std::env;

    use async_trait;
    use dotenvy::dotenv;
    use test_context::{test_context, AsyncTestContext};
    use tokio::sync::OnceCell;

    use super::*;

    struct TenancyTestContext {
        _value: String,
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "postgres")] {
            use sqlx::{postgres::PgPoolOptions, PgPool};
            static TEST_DB_POOL: OnceCell<PgPool> = OnceCell::const_new();
   
            async fn get_test_db_pool() -> &'static PgPool {
                TEST_DB_POOL
                    .get_or_init(|| async {
                        dotenv().expect(".env file not found");
                        let database_url: String =
                            env::var("POSTGRES_TEST_DATABASE_URL").expect("env missing POSTGRES_TEST_DATABASE_URL");
                        let pool: PgPool = PgPoolOptions::new()
                            .max_connections(5)
                            .connect(&database_url)
                            .await
                            .expect("Could not create postgres test db pool");
                        let _m = sqlx::migrate!("../axum-tenancy-postgres/migrations")
                            .run(&pool)
                            .await
                            .expect("Postgres Migration failed");
                        return pool;
                    })
                    .await
            }
        }
    }
        
    cfg_if::cfg_if! {
        if #[cfg(feature = "sqlite")] {
            use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
            static TEST_DB_POOL: OnceCell<SqlitePool> = OnceCell::const_new();
   
            async fn get_test_db_pool() -> &'static SqlitePool {
                TEST_DB_POOL
                    .get_or_init(|| async {
                        dotenv().expect(".env file not found");
                        let database_url: String =
                            env::var("SQLITE_TEST_DATABASE_URL").expect("env missing SQLITE_TEST_DATABASE_URL");
                        let pool: SqlitePool = SqlitePoolOptions::new()
                            .max_connections(5)
                            .connect(&database_url)
                            .await
                            .expect("Could not create sqlite test db pool");
                        let _m = sqlx::migrate!("../axum-tenancy-sqlite/migrations")
                            .run(&pool)
                            .await
                            .expect("Sqlite Migration failed");
                        return pool;
                    })
                    .await
            }
        }
    }

    #[async_trait::async_trait]
    impl AsyncTestContext for TenancyTestContext {
        async fn setup() -> TenancyTestContext {
            let _pool = get_test_db_pool();

            TenancyTestContext {
                _value: "hello world".to_string(),
            }
        }

        async fn teardown(self) {
            // no teardown at the moment
        }
    }

    #[test_context(TenancyTestContext)]
    #[tokio::test(flavor = "multi_thread")]
    async fn insert_user_no_dup_user_name(
        _tenancy_context: &mut TenancyTestContext,
    ) -> sqlx::Result<(), sqlx::Error> {
        let pool = get_test_db_pool();
        let mut tx: DbTransaction = pool.await.begin().await?;
        let user_result = insert(
            &mut tx,
            "Dave",
            "Dave Warnock",
            true,
            "dwarnock@test.com",
            "01234567891",
        )
        .await;
        assert_eq!(&user_result.is_ok(), &true);
        assert!(insert(
            &mut tx,
            "Dave",
            "not Dave Warnock",
            true,
            "dwarnock@test.com",
            "01234567891"
        )
        .await
        .is_err());

        Ok(())
    }

    #[test_context(TenancyTestContext)]
    #[tokio::test(flavor = "multi_thread")]
    async fn insert_user_no_dup_display_name(
        _tenancy_context: &mut TenancyTestContext,
    ) -> sqlx::Result<(), sqlx::Error> {
        let pool = get_test_db_pool();
        let mut tx: DbTransaction = pool.await.begin().await?;
        let user_result = insert(
            &mut tx,
            "Dave",
            "Dave Warnock",
            true,
            "dwarnock@test.com",
            "01234567891",
        )
        .await;
        assert_eq!(&user_result.is_ok(), &true);
        assert!(insert(
            &mut tx,
            "NotDave",
            "Dave Warnock",
            true,
            "dwarnock@test.com",
            "01234567891"
        )
        .await
        .is_err());

        Ok(())
    }

    #[test_context(TenancyTestContext)]
    #[tokio::test(flavor = "multi_thread")]
    async fn insert_then_check_load_user(
        _tenancy_context: &mut TenancyTestContext,
    ) -> sqlx::Result<(), sqlx::Error> {
        let pool = get_test_db_pool();
        let mut tx: DbTransaction = pool.await.begin().await?;
        let user_result = insert(
            &mut tx,
            "Dave",
            "Dave Warnock",
            true,
            "dwarnock@test.com",
            "01234567891",
        )
        .await;
        assert_eq!(&user_result.is_ok(), &true);

        let inserted_uuid = user_result.unwrap_or_default();
        assert_ne!(&inserted_uuid.to_string(), ""); // uuid must not be empty

        let load_result = load_by_id(&mut tx, inserted_uuid).await;
        assert_eq!(&load_result.is_ok(), &true); // load_by_id reult is ok

        let loaded_user = load_result.unwrap_or_default();
        assert_eq!(&loaded_user.user_id, &inserted_uuid);
        assert_eq!(&loaded_user.user_name.to_string(), &"Dave");
        assert_eq!(&loaded_user.display_name.to_string(), &"Dave Warnock");
        assert_eq!(&loaded_user.is_admin, &true);
        assert_eq!(&loaded_user.email.to_string(), &"dwarnock@test.com");
        assert_eq!(&loaded_user.mobile_phone.to_string(), &"01234567891");

        Ok(())
    }

    #[test_context(TenancyTestContext)]
    #[tokio::test(flavor = "multi_thread")]
    async fn insert_update_then_check(
        _tenancy_context: &mut TenancyTestContext,
    ) -> sqlx::Result<(), sqlx::Error> {
        let pool = get_test_db_pool();
        let mut tx: DbTransaction = pool.await.begin().await?;
        let insert_result = insert(
            &mut tx,
            "Dave",
            "Dave Warnock",
            true,
            "dwarnock@test.com",
            "01234567891",
        )
        .await;
        assert_eq!(&insert_result.is_ok(), &true);

        let inserted_uuid = insert_result.unwrap_or_default();
        assert_ne!(&inserted_uuid.to_string(), ""); // uuid must not be empty

        let update_result = update(
            &mut tx,
            &inserted_uuid,
            "not Dave",
            "not Dave Warnock",
            false,
            "not dwarnock@test.com",
            "6601234567891",
        )
        .await;
        assert_eq!(&update_result.is_ok(), &true);
        let rows_affected = update_result.unwrap();
        assert_eq!(rows_affected, 1);

        let load_result = load_by_id(&mut tx, inserted_uuid).await;
        assert_eq!(&load_result.is_ok(), &true); // load_by_id reult is ok

        let loaded_user = load_result.unwrap_or_default();
        assert_eq!(&loaded_user.user_id, &inserted_uuid);
        assert_eq!(&loaded_user.user_name.to_string(), &"not Dave");
        assert_eq!(&loaded_user.display_name.to_string(), &"not Dave Warnock");
        assert_eq!(&loaded_user.is_admin, &false);
        assert_eq!(&loaded_user.email.to_string(), &"not dwarnock@test.com");
        assert_eq!(&loaded_user.mobile_phone.to_string(), &"6601234567891");

        Ok(())
    }

    #[test_context(TenancyTestContext)]
    #[tokio::test(flavor = "multi_thread")]
    async fn insert_two_check_load_all(
        _tenancy_context: &mut TenancyTestContext,
    ) -> sqlx::Result<(), sqlx::Error> {
        let pool = get_test_db_pool();
        let mut tx: DbTransaction = pool.await.begin().await?;
        let user_result1 = insert(
            &mut tx,
            "zDave",
            "Dave Warnock",
            true,
            "dwarnock@test.com",
            "01234567891",
        )
        .await;
        assert_eq!(&user_result1.is_ok(), &true);

        let inserted_uuid1 = user_result1.unwrap_or_default();
        assert_ne!(&inserted_uuid1.to_string(), ""); // uuid must not be empty

        let user_result2 = insert(
            &mut tx,
            "Dave2",
            "Dave Warnock2",
            false,
            "dwarnock@test.com2",
            "012345678912",
        )
        .await;
        assert_eq!(&user_result2.is_ok(), &true);

        let inserted_uuid2 = user_result2.unwrap_or_default();
        assert_ne!(&inserted_uuid2.to_string(), ""); // uuid must not be empty

        let load_result = load_all_sorted(&mut tx, UserSort::UserName, SortDirection::Asc).await;
        assert_eq!(&load_result.is_ok(), &true); // load_by_id reult is ok
        let vec_users = &load_result.unwrap();
        assert_eq!(&vec_users.len(), &2usize); // load_by_id reult is ok

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

    #[test_context(TenancyTestContext)]
    #[tokio::test(flavor = "multi_thread")]
    async fn insert_two_check_sort_desc(
        _tenancy_context: &mut TenancyTestContext,
    ) -> sqlx::Result<(), sqlx::Error> {
        let pool = get_test_db_pool();
        let mut tx: DbTransaction = pool.await.begin().await?;
        let user_result1 = insert(
            &mut tx,
            "zDave",
            "Dave Warnock",
            true,
            "dwarnock@test.com",
            "01234567891",
        )
        .await;
        assert_eq!(&user_result1.is_ok(), &true);

        let inserted_uuid1 = user_result1.unwrap_or_default();
        assert_ne!(&inserted_uuid1.to_string(), ""); // uuid must not be empty

        let user_result2 = insert(
            &mut tx,
            "Dave2",
            "Dave Warnock2",
            false,
            "dwarnock@test.com2",
            "012345678912",
        )
        .await;
        assert_eq!(&user_result2.is_ok(), &true);

        let inserted_uuid2 = user_result2.unwrap_or_default();
        assert_ne!(&inserted_uuid2.to_string(), ""); // uuid must not be empty

        let load_result =
            load_all_sorted(&mut tx, UserSort::DisplayName, SortDirection::Desc).await;
        assert_eq!(&load_result.is_ok(), &true); // load_by_id reult is ok
        let vec_users = &load_result.unwrap();
        assert_eq!(&vec_users.len(), &2usize); // load_by_id reult is ok

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
