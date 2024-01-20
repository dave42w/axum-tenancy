# axum-tenancy
Multi tenant support for [Axum](https://crates.io/crates/axum) web apps

Making it easy to get started on a multi-tenant web app, builds on [axum-login](https://crates.io/crates/axum-login)

Examples and the initial focus for quickstart use [Sqlx](https://crates.io/crates/sqlx) with [SQLite](https://www.sqlite.org/index.html) and [Askama](https://crates.io/crates/askama)

Comes out of [PrayerOfHannah](https://github.com/dave42w/PrayerOfHannah)

## Scope

There are two main parts to axum-tenancy.

### Admin

Used to manage the Users, Tenants and User access to Tenants.

Initially axum-tenancy will create and manage User, Tenant and UserTenant tables in SQLite. In the future it would be good to work with an existing application User table and with other dbms.

Initially axum-tenancy will provide html pages (using [Askama](https://crates.io/crates/askama) and [Htmx](https://htmx.org/)). In the future it would be good to allow the application to provide it's own templates (probably first make the existing templates so thay can be included in an application page) or use a different template engine.

### Middleware

axum-tenancy includes [Tower Middleware](https://crates.io/crates/tower), the application decides which routes to protect by the axum-tenancy layer. The axum-tenancy layer restricts users to the Tenants that they have access to.

## License

MIT License

Copyright (c) 2024 Dave Warnock

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

