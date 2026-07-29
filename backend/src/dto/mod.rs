pub mod user;

trait Validate {
    fn validate(&self) -> Result<(), String>;
}
