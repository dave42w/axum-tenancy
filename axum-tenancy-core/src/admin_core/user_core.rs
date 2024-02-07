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

pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC"
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
}
