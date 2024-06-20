/// Members of this trait can be validated for errors when created programatically.
pub trait Validate {
    /// This function is called when a chunk is pushed to a document.
    fn validate(&self) -> Result<(), crate::InternalError>;
}
