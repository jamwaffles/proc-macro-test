pub trait EventData {
    fn namespaced_type(&self) -> &'static str;
}
