pub trait Metrics {
    fn increment(
        &self,
        name: &str,
        value: f64,
    );

    fn observe(
        &self,
        name: &str,
        value: f64,
    );
}