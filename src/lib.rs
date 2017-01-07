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

pub trait Channel<ServiceType, ObjectType> where
        ServiceType : Service,
        ObjectType  : Object {

    /// Get identifier of a requester-object.
    fn requester_id(&self) -> ObjectType::Id;

    /// Get identifier of a requested service.
    fn service_id(&self) -> ServiceType::Id;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
