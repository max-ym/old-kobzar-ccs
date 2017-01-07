/// Object is an instance of some program that lives in computer
/// memory and may perform some tasks.
pub trait Object {

    /// Type that is uniquely identifying objects in the system.
    type ObjectId;

    /// Get object unique identifier.
    fn id(&self) -> Self::ObjectId;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
