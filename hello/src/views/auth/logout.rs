use actix_web::HttpResponse;

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; cahrset=utf-8")
        .body("<html>\
            <script>\
            localStorage.removeItem('user-token');\
            window.location.origin);\
            </script>\
            </html>")
}
