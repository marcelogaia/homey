/// Fetches room temperature from a Smart Thermometer
#[allow(dead_code)]
struct FetchRoomTemperature {
    fail: bool,
}

#[allow(dead_code)]
impl FetchRoomTemperature {

    fn new(fail: bool) -> Self {
        Self { fail }
    }

    /// Executes the use case.
    ///
    /// Returns the room temperature as a floating-point number in degrees Celsius.
    fn execute(&self) -> Result<f32, Box<dyn std::error::Error>> {
        if self.fail {
            Err("Temperature fetch failed".into())
        } else {
            Ok(20.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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