/// Fetches room temperature from a Smart Thermometer
#[derive(Clone)]
pub struct FetchRoomTemperature {
    fail: bool,
}

impl FetchRoomTemperature {
    pub fn new(fail: bool) -> Self {
        Self { fail }
    }

    /// Executes the use case.
    ///
    /// Returns the room temperature as a floating-point number in degrees Celsius.
    pub fn execute(&self) -> Result<f32, Box<dyn std::error::Error>> {
        if self.fail {
            Err("Temperature fetch failed".into())
        } else {
            Ok(20.0)
        }
    }
}
