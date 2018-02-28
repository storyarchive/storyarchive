pub mod hub;

#[derive(Clone, Serialize, Deserialize)]
pub struct Page<'s> {
    pub css: &'s str,
    pub hub: Option<hub::Hub>,
}