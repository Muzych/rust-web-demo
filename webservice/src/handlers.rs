use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::models::Course;
use chrono::Utc;

pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = 
        format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}


pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    #[actix_rt::test]
    async fn post_course_test() {

        dotenv().ok();
        let db_url = env::var("DATABASE_URI").expect("DATABASE_URI is not set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();


        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test Course".into(),
            id: None,
            time: None,
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {

        dotenv().ok();
        let db_url = env::var("DATABASE_URI").expect("DATABASE_URI is not set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState { 
            health_check_response: "".to_string(), 
            visit_count: Mutex::new(0),
            db: db_pool
         });
         let teacher_id: web::Path<(usize,)> = web::Path::from((1,));
         let resp = get_courses_for_teacher(app_state, teacher_id).await;
         assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URI").expect("DATABASE_URI is not set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState { 
            health_check_response: "".to_string(), 
            visit_count: Mutex::new(0),
            db: db_pool
         });
         let params: web::Path<(usize, usize)> = web::Path::from((1,1));
         let resp = get_course_detail(app_state, params).await;
         assert_eq!(resp.status(), StatusCode::OK);
    }
}