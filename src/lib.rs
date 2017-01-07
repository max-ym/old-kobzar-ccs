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

    /// Get service identifier.
    fn id(&self) -> Self::Id;
}

pub trait Channel<ServiceType, ObjectType> where
        ServiceType : Service,
        ObjectType  : Object {

    /// Get requester-object reference.
    fn requester(&self) -> &ObjectType;

    /// Get requested service reference.
    fn service(&self) -> &ServiceType;
}

pub trait Page {
    // Empty currently.
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
