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

