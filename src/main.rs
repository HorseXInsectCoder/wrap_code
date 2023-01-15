use std::collections::HashMap;
use std::env;
use std::fs::create_dir;
use serde_json::{json, Number, Value};
use warp::Filter;
use warp::reply::Json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let log = warp::log("apis");

    let hi = warp::get()
        .and(warp::path("hello"))
        .map(|| "hi");

    // 直接用一个接口统一如 rest_get, rest_list, rest_create 这几个接口
    let apis = hi.or(rest_api()).with(log);

    warp::serve(apis).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}

// 要返回实现了Filter的类型，且里面的范型要实现 Reply，错误是 Rejection，且必须是Clone的
fn rest_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base_url = warp::path("rest");

    // url: rest/xx
    let get = base_url
        .and(warp::get())
        .and(warp::path::param())
        .and_then(rest_get);

    let list = base_url
        .and(warp::get())
        .and(warp::path::end())
        .and_then(rest_list);

    let create = base_url
        .and(warp::post())
        .and(warp::body::json())        // 从请求体读取JSON
        .and_then(rest_create);


    get.or(list).or(create)
}

// 注意用的是 warp::reply::Json;
async fn rest_get(id: i32) -> Result<Json, warp::Rejection> {
    let some_thing = json!({
        "id": id,
        "name": format!("name: {}", id)
    });
    let some_thing_warp = warp::reply::json(&some_thing);
    Ok(some_thing_warp)
}

async fn rest_list() -> Result<Json, warp::Rejection> {
    let res = json!([
        {"id": 1,"status": "ok"},
        {"id": 2,"status": "err"},
        {"id": 3,"status": "aa"},
        {"id": 4,"status": "bb"},
        {"id": 5,"status": "cc"},
    ]);

    let some_thing_warp = warp::reply::json(&res);
    Ok(some_thing_warp)
}

// 从请求体里接收JSON
// 参数是 serde_json::Value;
async fn rest_create(data: Value) -> Result<Json, warp::Rejection> {
    let mut data: serde_json::Value = serde_json::from_value(data).unwrap();

    let data_id: i32 = serde_json::from_value(data["id"].clone()).unwrap();
    println!("{}", data_id);

    data["id"] = Value::Number(Number::from(data_id + 1));

    // 把接收到的JSON直接返回出去
    Ok(warp::reply::json(&data))
}