# 🚀 A Near-Realworld Leptos Web App with Axum and PostgreSQL Backend


<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Background

A full-stack application built with [Leptos](https://leptos.dev/) and Postgres, implementing a RealWorld blog platform with **session-based authentication**.


This repository builds upon the original [realworld-app-leptos-axum](https://github.com/santhosh7403/realworld-app-leptos-axum) by replacing the JWT-based authentication mechanism with a more traditional session-based approach using [axum_session](https://crates.io/crates/axum_session) and [axum_session_auth](https://crates.io/crates/axum_session_auth) crates.

## Session-Based Authentication

Rather than using stateless JWT tokens, this implementation leverages server-side session management where:
- **Session State**: User sessions are managed server-side, providing better control and security
- **axum_session**: Handles session storage, retrieval, and lifecycle management
- **axum_session_auth**: Provides authentication middleware and user extraction utilities
- **Security Benefits**: Sessions can be invalidated immediately, user context is always current, and CSRF protection is naturally built-in

This approach is particularly well-suited for traditional web applications and demonstrates an alternative to JWT for Rust web development.

I started with the original Leptos/Postgres implementation and adapted it to use session-based authentication as an exploration of different auth patterns in Rust fullstack applications.


Before proceeding, you can view the application's functionality via the[ screenshots here ](https://github.com/santhosh7403/axum-session-auth-realworld-app-leptos-postgres/blob/main/App_Screenshots.md).

---

## 🛠️ Key Technologies & Features

This application leverages the following core technologies and features:

* Leptos
* axum
* Server-Side Rendering (SSR)
* postgres
* fts (Full-Text Search)
* Modal Windows
* axum_session (session based auth)
* uuid
* tailwindcss
* fontawesome icons
* nanoid (password reset token)

---

## ⚙️ Install and Run

**Prerequisites**

By default, `cargo-leptos` requires the **Rust nightly** toolchain and several cargo extensions. If you encounter issues, ensure these tools are installed. Consult the [ rustup documentation ](https://rustup.rs) for detailed instructions.

### Required Tools

Ensure the following Rust toolchains and dependencies are installed:


1.  `rustup toolchain install nightly --allow-downgrade` (Installs or ensures the **Rust nightly** toolchain is available)
2.  `rustup update` (Updates all installed Rust toolchains to their latest version)
3.  `rustup target add wasm32-unknown-unknown` (Adds the target necessary for compiling Rust to WebAssembly)
4.  `cargo install cargo-generate` (Installs the project templating tool)
5.  `cargo install cargo-leptos --locked` (Installs the essential Leptos build tool)


### Clone Repository

Clone the repository to your local machine:

```bash
git clone https://github.com/santhosh7403/axum-session-auth-realworld-app-leptos-postgres.git]
cd axum-session-auth-realworld-app-leptos-postgres
```


### Database Initialization

1. `source .env` - to set the DATABASE_URL env variable

2, Follow the steps in [ README_DATABASE.md ](https://github.com/santhosh7403/axum-session-auth-realworld-app-leptos-postgres/blob/main/README_DATABASE.md) to initialize the database schema and data.

### Run Application

You may now build and run the application:

```bash
cargo leptos watch 
# OR
cargo leptos serve
```
---

### Application access

Once the application has started successfully, access it via your web browser at [ localhost:3000 ](http://localhost:3000/)

Sample application screens.
<img width="1924" height="1033" alt="image" src="https://github.com/user-attachments/assets/2eb4d1ab-d80a-46a7-8cbd-768692b7e435" />
<img width="1924" height="1033" alt="image" src="https://github.com/user-attachments/assets/e4c10d56-48ec-4432-96a4-6c330d27dd0f" />
<img width="1924" height="1033" alt="image" src="https://github.com/user-attachments/assets/b74cd6d7-f21a-4ee8-bf6b-50794154a0ab" />





More screenshots are [ available here ](https://github.com/santhosh7403/axum-session-auth-realworld-app-leptos-postgres/blob/main/App_Screenshots.md)

---

### Sample User Data

The application is pre-populated with sample users and data for immediate testing and demonstration.

1.   Available Users: user1 to user5

2.   Password: The password is the same as the username (e.g., user1 has a password of user1).

To remove this default data, delete the base data files within the `./migrations` folder and follow the database setup steps outlined in the [README_DATABASE.md](https://github.com/santhosh7403/axum-session-auth-realworld-app-leptos-postgres/blob/main/README_DATABASE.md).

---

### PostgreSQL Full Text Search

The application features a robust full-text search capability powered by PostgreSQL Full Text Search, which indexes three key fields from the `articles` table. For developers interested in the implementation or experimenting with different search methodologies, comprehensive documentation is available in the PostgreSQL [documentation here. ](https://www.postgresql.org/docs/17/textsearch.html)

Another blog post on [PostgreSQL Full Text Search](https://iniakunhuda.medium.com/postgresql-full-text-search-a-powerful-alternative-to-elasticsearch-for-small-to-medium-d9524e001fe0)

## 🏗️ Other Variants

If you are looking for this same application with different frameworks or databases, check out these versions:


| Framework | Database | Auth Type | Auth Crates | Special Feature | Repository |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Leptos** | PostgreSQL | Session | axum_session, axum_session_auth | | *This Repository* |
| **Dioxus** | SQLite | Session | axum_session, axum_session_auth | | [View Repo](https://github.com/santhosh7403/axum-session-auth-realworld-app-dioxus-sqlite) |
| **Dioxus** | SQLite | Session | tower_sessions, axum_login | superadmin, fine grained authorization | [View Repo](https://github.com/santhosh7403/tower-sessions-axum-login-realworld-app-dioxus-sqlite) |
| **Dioxus** | SQLite | PASETO | pasetors | superadmin, fine grained authorization | [View Repo](https://github.com/santhosh7403/paseto-auth-realworld-app-dioxus-sqlite) |
| **Leptos** | PostgreSQL | JWT | jsonwebtoken | | [View Repo](https://github.com/santhosh7403/realworld-app-leptos-axum) |
| **Leptos** | SQLite | JWT | jsonwebtoken | | [View Repo](https://github.com/santhosh7403/realworld-app-leptos-axum-sqlite) |
| **Dioxus** | SQLite | JWT | jsonwebtoken | | [View Repo](https://github.com/santhosh7403/realworld-app-dioxus-sqlite) |



## 🙏 Inspiration and Acknowledgements

The foundational structure of this application is derived from the realworld example by [Bechma/realworld-leptos](https://github.com/Bechma/realworld-leptos), with appreciation to any antecedent projects.

This particular version was initiated during the transition from Leptos 0.6 to 0.7 as a personal learning exercise. It has since undergone significant experimentation and refinement, including:

*   A complete user interface redesign utilizing tailwindcss and fontawesome icons.

*   Implementation of modal windows and re-wired page navigation.

*   Integration of Postgres FTS for comprehensive full-text search capabilities.

*   An updated, non-reloading pagination method for search results.

*   Dark mode styling and user preference persistence.

*   Implementation of Session based auth.
