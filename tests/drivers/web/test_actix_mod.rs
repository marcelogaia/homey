
#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use actix_rt;
    use homey::use_cases::fetch_room_temperature::FetchRoomTemperature;
    use homey::drivers::web::actix_mod::get_room_temperature;

    #[actix_rt::test]
    async fn test_get_room_temperature() {
        let use_case = FetchRoomTemperature::new(false);
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(use_case))
                .route("/room_temperature", web::get().to(get_room_temperature))
        ).await;

        let req = test::TestRequest::get().uri("/room_temperature").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_get_room_temperature_body() {
        let use_case = FetchRoomTemperature::new(false);
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(use_case)) // Use the imported Data struct
                .route("/room_temperature", web::get().to(get_room_temperature))
        ).await;

        let req = test::TestRequest::get().uri("/room_temperature").to_request();
        let resp = test::call_service(&mut app, req).await;

        let body: String = test::read_body_json(resp).await; // Add type annotation for deserialization
        assert_eq!(body, "Room temperature: 20");
    }

    #[actix_rt::test]
    async fn test_get_room_temperature_error() {
        let use_case = FetchRoomTemperature::new(true);  // Create a failing use case
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(use_case))
                .route("/room_temperature", web::get().to(get_room_temperature))
        ).await;

        let req = test::TestRequest::get().uri("/room_temperature").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_client_error());
    }
}