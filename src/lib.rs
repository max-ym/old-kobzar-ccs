/// Object is an instance of some program that lives in computer
/// memory and may perform some tasks.
pub trait Object {

    /// Type that is uniquely identifying objects in the system.
    type ObjectId;

    /// Get object unique identifier.
    fn id(&self) -> Self::ObjectId;
}

pub trait Service  {

    /// Type that is uniquely identifying services in the system.
    type ServiceId;

    /// Get serivce identifier.
    fn id(&self) -> Self::ServiceId;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
