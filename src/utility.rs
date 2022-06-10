pub struct EcstasyUtility;

impl EcstasyUtility {
    pub fn human_size(size: f64, power: f64, metric: &'static str) -> String {
        format!("{:.2} {}", size / 1024f64.powf(power), metric)
    }
}
