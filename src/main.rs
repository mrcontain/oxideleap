/*
 * @Author: mrcontain 1916985079@qq.com
 * @Date: 2025-02-11 14:28:47
 * @LastEditors: mrcontain 1916985079@qq.com
 * @LastEditTime: 2025-04-04 17:35:09
 * @FilePath: \oxideleap\src\main.rs
 * @Description:
 *
 * Copyright (c) 2025 by ${git_name_email}, All Rights Reserved.
 */
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use sea_orm::ConnectOptions;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, level_filters::LevelFilter, warn};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{filter::FilterExt, fmt::Layer, prelude::*, EnvFilter};

const _MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/oxideleap";
#[tokio::main]
async fn main() {
    let _opt = ConnectOptions::new("mysql://test:123456@localhost/blog");
    //create the filter layer with the info level and use th levelfilter to filter out the logs
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);

    //output the error logs to the error.log
    // and only the info logs and warn logs to the oxideleap.log
    let filter_appender = rolling::daily("logs", "oxideleap.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(filter_appender);
    let filter_layer = Layer::new()
        .with_writer(non_blocking)
        .with_filter(EnvFilter::from_default_env().add_directive("oxideleap=info".parse().unwrap()));
    let error_appender = rolling::daily("logs", "error.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(error_appender);
    let error_layer = Layer::new()
        .with_writer(non_blocking)
        .with_filter(LevelFilter::ERROR);

    tracing_subscriber::registry()
        .with(layer)
        .with(filter_layer)
        .with(error_layer)
        .init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/{*key}", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("OxideLeap started! bind {}", "0.0.0.0:3000");
    debug!("hahahah");
    warn!("test");
    error!("ssss");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(Path(path): Path<String>) -> String {
    format!("Hello, {}", path)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
