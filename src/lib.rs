/// Object is an instance of some program that lives in computer
/// memory and may perform some tasks.
pub trait Object {

    /// Type that is uniquely identifying objects in the system.
    type Id;

    /// Get object unique identifier.
    fn id(&self) -> Self::Id;
}

pub trait Service  {

    /// Type that is uniquely identifying services in the system.
    type Id;

    /// Get serivce identifier.
    fn id(&self) -> Self::Id;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
