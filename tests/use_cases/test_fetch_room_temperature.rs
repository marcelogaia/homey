#[cfg(test)]
mod tests {
    use homey::use_cases::fetch_room_temperature::FetchRoomTemperature;

    #[test]
    fn test_execute_returns_specific_temperature() {
        let use_case = FetchRoomTemperature::new(false);
        let result = use_case.execute();
        assert!(result.is_ok());

        let temperature = result.unwrap();
        assert_eq!(temperature, 20.0);
    }

    #[test]
    fn test_execute_handles_failure() {
        let use_case = FetchRoomTemperature::new(true);
        let result = use_case.execute();
        assert!(result.is_err());
    }
}
