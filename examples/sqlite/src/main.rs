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
