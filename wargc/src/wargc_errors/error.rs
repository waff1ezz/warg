pub trait Error {
    fn throw(&self) -> !;
}