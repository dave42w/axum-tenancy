pub fn initialize() {
    println!("Initialize axum-tenancy");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_without_panic() {
        initialize();
    }
}
