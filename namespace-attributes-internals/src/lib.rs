pub trait EventData {
    fn get_namespace_and_type(&self) -> String;
}
