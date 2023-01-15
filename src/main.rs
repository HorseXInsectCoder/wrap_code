use std::collections::HashMap;
use std::env;
use warp::Filter;

const WEB_DIR: &str = "web/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 引入环境变量
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let log = warp::log("basic");

    // 这种方式同时能用于get和post。但用宏写会比较容易出bug，写比较简单的可以这么写
    let greet = warp::path!("basic"/String/i32)
        .map(|name, age| {
            return format!("name: {}, age: {}", name, age); // 返回时要把i32转成string，因为i32没有实现 Reply trait
        });

    // 注：map是同步的，and_then才是异步的
    let add = warp::path!("add"/i32/i32)
        .map(|a, b| return format!("res: {}", a + b));

    // 不使用宏的方式，获取url参数
    // path/$name?$a=$b
    let items = warp::get()
        .and(warp::path("items"))
        .and(warp::path::param::<String>())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(get_items);

    // 同时支持多个链接，且限定只能用get访问
    let apis = warp::get().and(greet.or(add).or(items)).with(log);


    let dir_static = warp::fs::dir(WEB_DIR);
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_DIR)));
    let static_route = dir_static.or(index);

    let routes = static_route.or(apis);

    warp::serve(routes).run(([127, 0, 0, 1], 8181)).await;

    Ok(())
}

// 处理从url获取到的参数
async fn get_items(
    param: String,
    param_map: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("get {}: {:?}", param, param_map))
}