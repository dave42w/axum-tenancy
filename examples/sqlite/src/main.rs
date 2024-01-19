use dotenvy::dotenv;
use axum_tenancy;

fn main() {
    println!("Hello, world!");
    dotenv().expect(".env file not found");
    axum_tenancy::initialize();
}


#[cfg(test)]
mod tests {
    use std::env;
    use dotenvy::dotenv;

    #[test]
    fn sqlite_database_url_correct() {
        dotenv().expect(".env file not found");

        let database_url: String = env::var("DATABASE_URL").expect(".env missing DATABASE_URL");
        assert_eq!(database_url, "sqlite:axum-tenancy.sqlite?mode=rwc".to_string());
    }

    #[test]
    fn axum_tenancy_initialise_ok() {
        axum_tenancy::initialize(); // would panic if it fails
    }
}
