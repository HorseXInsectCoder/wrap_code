
use warp::Filter;

const XAUTH: &str = "X-Auth-Token";

pub fn auth() -> impl Filter<Extract = (ContextUser, ), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::<String>(XAUTH))
        .and_then(|xauth: String| async move {
            if !xauth.starts_with("ok:") {
                return Err(warp::reject::custom(AuthError))
            }
            // Ok::<(), warp::Rejection>(())

            // 注意不要使用同步的map
            if let Some(id) = xauth.split(":").skip(1).next().and_then(|v| v.parse::<i64>().ok()) {
                Ok::<ContextUser, warp::Rejection>(ContextUser { id })
            } else {
                return Err(warp::reject::custom(AuthErrorNumberNeed))       // 如果"ok:"后面没传数字，就报AuthErrorNumberNeed
            }
        })
}

// 封装header信息
pub struct ContextUser {
    pub id: i64
}

#[derive(Debug)]
struct AuthError;

// 只要目标结构体实现了Debug，其余东西Reject都已经实现好了
impl warp::reject::Reject for AuthError {}

#[derive(Debug)]
struct AuthErrorNumberNeed;

impl warp::reject::Reject for AuthErrorNumberNeed {}