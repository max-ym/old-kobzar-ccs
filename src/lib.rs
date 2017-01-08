/// Object is sort of process in Kobzar. It is an instanse of some
/// program that is currently running on the system, or residing in
/// the RAM. The CCS object may request services of other objects which
/// either already are loaded or can be loaded to reply on the request.
pub trait Object<S: Service<Self>>: Sized {

    /// Use to separate different objects in the system so that
    /// it was possible to definitely recognize one object
    /// among all others.
    type Id;

    /// Get object identifier. Each object in one CCS network has
    /// unique identifier.
    fn id(&self) -> Self::Id;

    /// Get information about the service by given identifier.
    /// If object has no service with given identifier, None will
    /// be returned.
    fn service_by_id(&self, id: &S::Id) -> Option<&S>;

    /// Get object of current running application. When this application
    /// calls this function, it gets a self object.
    fn myself() -> Self;
}

/// Service is requested by the Object. Service is used to update some
/// data, create or delete it, make some calculations or make any other
/// change to the system. It can be provided by a single program on the
/// system or by a group of different programs. CCS decides which
/// program may serve the request or may decline contributing to
/// request transferer. This trait identifies single service that can
/// be provided by any program in the system.
pub trait Service<O: Object<Self>>: Sized {

    /// Use to separate different services in the network so that
    /// it was possible to definitely recognize one service
    /// among all others.
    type Id;

    /// Get service identifier. Each service in one CCS network has
    /// unique identifier.
    fn id(&self) -> Self::Id;

    /// Request a service. If any object in CCS network can provide
    /// such service, then channel is created.
    fn request<RC: RequesterChannel<O, Self>>(&self) -> Option<RC>;

    /// Attempt to register new service that current object is ready to
    /// provide. The Fn argument is a function that is runned when
    /// servie is requested by an object in the system.
    fn register<SC: ServerChannel<O, Self>>(entry: Fn(SC))
            -> Result<Self, RegistrationErr>;

    /// Try to register service that the object that called this
    /// function is ready to provide. The difference from 'register'
    /// function is that this one tries to make the service unique
    /// in the CCS network. That is, any other object can't provide
    /// the same service at the same time. If this service is
    /// already registered, system will decline in registering this
    /// service.
    ///
    /// This is useful for security reasons. For example, we
    /// have got some operating system and a Memory Server running on it.
    /// It provides some service to allocate memory. Only
    /// Memory Server is allowed to allocate memory and since it
    /// starts very early at system initialization, it uniquely registers
    /// its services so no other objects in the system later after
    /// booting couldn't succeed in service interception.
    fn register_unique<SC: ServerChannel<O, Self>>(entry: Fn(SC))
            -> Result<Self, RegistrationErr>;
}

/// Channel is a connection of the requester-object that requests the
/// service and the provider object. Data transfer is performed by
/// implementation of this trait.
pub trait Channel<O, S>: Sized
        where O: Object<S>, S: Service<O> {

    /// Get object which requested the service.
    fn requester(&self) -> &O;

    /// Get service that was requested.
    fn service(&self) -> &S;
}

/// A channel handle of a server.
pub trait ServerChannel<O, S>: Channel<O, S>
        where O: Object<S>, S: Service<O> {
}

/// A channel handle of a requester.
pub trait RequesterChannel<O, S>: Channel<O, S>
        where O: Object<S>, S: Service<O> {
}

/// Error that can appear when new service is being registered.
pub enum RegistrationErr {

    /// The service couldn't be registered because the same
    /// service is uniquely provided by some object.
    UniquelyRegistered,

    /// When attempting to uniquely register some service and
    /// when the same service is already registered in the system,
    /// CCS network can't register this service uniquely. Not until
    /// all the same services are closed.
    AlreadyRegistered
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
