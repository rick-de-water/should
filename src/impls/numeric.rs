use crate::{caller_name::get_caller_name, expected::Expected, ShouldBeApproximatelyEqual};

impl ShouldBeApproximatelyEqual<f32> for f32 {
    fn should_be_approx(&self, expected: impl Expected<f32>, tolerance: impl Expected<f32>) {
        let other = expected.value();
        let tolerance = tolerance.value();
        let high = other + tolerance;
        let low = other - tolerance;
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());

        if *self > high || *self < low {
            panic!("{} should be between {:?} and {:?} but was {:?}", caller_name, low, high, self)
        }
    }

    fn should_not_be_approx(&self, expected: impl Expected<f32>, tolerance: impl Expected<f32>) {
        let other = expected.value();
        let tolerance = tolerance.value();
        let high = other + tolerance;
        let low = other - tolerance;
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());

        if *self <= high && *self >= low {
            panic!("{} should not be between {:?} and {:?} but was {:?}", caller_name, low, high, self)
        }
    }
}

impl ShouldBeApproximatelyEqual<f64> for f64 {
    fn should_be_approx(&self, expected: impl Expected<f64>, tolerance: impl Expected<f64>) {
        let other = expected.value();
        let tolerance = tolerance.value();
        let high = other + tolerance;
        let low = other - tolerance;
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());

        if *self > high || *self < low {
            panic!("{} should be between {:?} and {:?} but was {:?}", caller_name, low, high, self)
        }
    }

    fn should_not_be_approx(&self, expected: impl Expected<f64>, tolerance: impl Expected<f64>) {
        let other = expected.value();
        let tolerance = tolerance.value();
        let high = other + tolerance;
        let low = other - tolerance;
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());

        if *self <= high && *self >= low {
            panic!("{} should not be between {:?} and {:?} but was {:?}", caller_name, low, high, self)
        }
    }
}