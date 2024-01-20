use sqlx;

pub fn initialize() {
    println!("Initialize axum-tenancy");
}

#[cfg(test)]
use sqlx::postgres::PgPool;

#[test]
fn initialize_without_panic() {
    initialize();
}

#[sqlx::test]
async fn add_appuser(pool: PgPool) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO AppUser (
            id, 
            user_name, 
            hash_password, 
            display_name, 
            is_admin, 
            email,
            mobile_phone
        )
        VALUES (
            'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 
            'Dave42W',
            '0',
            'Dave Warnock',
            TRUE,
            'dwarnock@gmail.com',
            '07886 553376'
        )
        "#
    )
    .execute(&pool)
    .await?;
    Ok(())
    
}

