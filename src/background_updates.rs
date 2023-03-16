mod get;
pub use get::*;

mod enabled;
pub use enabled::*;

mod run;
pub use run::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundUpdateStatus {
    pub name: String,
    pub total_item_count: u64,
    pub total_duration_ms: f64,
    pub average_items_per_ms: f64,
}
