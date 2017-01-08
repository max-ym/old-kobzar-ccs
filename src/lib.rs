/// Object is sort of process in Kobzar. It is an instanse of some
/// program that is currently running on the system, or residing in
/// the RAM. The CCS object may request services of other objects which
/// either already are loaded or can be loaded to reply on the request.
pub trait Object<S: Service>: Sized {

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

    /// The object that called this function quits.
    /// All allocated resources are freed. All services registered
    /// by the object are removed.
    fn quit() -> !;
}

/// Service is requested by the Object. Service is used to update some
/// data, create or delete it, make some calculations or make any other
/// change to the system. It can be provided by a single program on the
/// system or by a group of different programs. CCS decides which
/// program may serve the request or may decline contributing to
/// request transferer. This trait identifies single service that can
/// be provided by any program in the system.
pub trait Service: Sized {

    /// Use to separate different services in the network so that
    /// it was possible to definitely recognize one service
    /// among all others.
    type Id;

    /// Get service identifier. Each service in one CCS network has
    /// unique identifier.
    fn id(&self) -> Self::Id;

    /// Look for service with given identifier in the CCS network.
    fn get_by_id(id: Id) -> Option<Self>;

    /// Request a service. If any object in CCS network can provide
    /// such service, then channel is created.
    fn request<O, RC>(&self) -> Option<RC>
        where O     : Object<Self>,
              RC    : RequesterChannel<O, Self>;

    /// Attempt to register new service that current object is ready to
    /// provide.
    fn register<O, OS, SC>(reg_form: RegistrationForm<O, Self, SC>)
        -> Result<OS, RegistrationErr>
        where   O   : Object<Self>,
                OS  : OwnedService<Id = Self::Id>,
                SC  : ServerChannel<O, Self>;

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
    fn register_unique<O, OS, SC>
        (reg_form: RegistrationForm<O, Self, SC>)
        -> Result<OS, RegistrationErr>
        where   O   : Object<Self>,
                OS  : OwnedService<Id = Self::Id>,
                SC  : ServerChannel<O, Self>;
}

/// OwnedService is received only by the object that registered
/// this service. Using this handle service provider-object can
/// discontinue service or do other owner-related stuff.
pub trait OwnedService: Sized + Service {

    /// Notify the system that object doesn't provide selected service
    /// no more. This function returns RegistrationForm so
    /// that discontinued service could be registered again.
    fn discontinue<O, SC>(self) -> RegistrationForm<O, Self, SC>
        where   O   : Object<Self>,
                SC  : ServerChannel<O, Self>;
}

pub struct RegistrationForm<O, S, SC>
        where O     : Object<S>,
              S     : Service,
              SC    : ServerChannel<O, S>
{
    _a      : std::marker::PhantomData<O>,

    /// Entry point. When service is requested, execution starts
    /// from given function.
    pub entry   : fn(SC),

    /// Identifier of the service.
    pub id      : S::Id,
}

impl<O, S, SC> RegistrationForm<O, S, SC>
        where O     : Object<S>,
              S     : Service,
              SC    : ServerChannel<O, S>
{

    /// Create new registration form.
    ///
    /// 'entry' argument is the entry function that will be called
    /// when service will be requested.
    ///
    /// 'id' argument is used to identify this service. Other
    /// objects will request this service from network by given
    /// identifier.
    pub fn new (
        entry   : fn(SC),
        id      : S::Id
    ) -> Self {
        RegistrationForm {
            _a      : std::marker::PhantomData,
            entry   : entry,
            id      : id
        }
    }
}

/// Channel is a connection of the requester-object that requests the
/// service and the provider object. Data transfer is performed by
/// implementation of this trait.
pub trait Channel<O, S>: Sized
        where O: Object<S>, S: Service {

    /// Get object which requested the service.
    fn requester(&self) -> &O;

    /// Get service that was requested.
    fn service(&self) -> &S;
}

/// A channel handle of a server.
pub trait ServerChannel<O, S>: Sized + Channel<O, S>
        where O: Object<S>, S: Service {
}

/// A channel handle of a requester.
pub trait RequesterChannel<O, S>: Sized + Channel<O, S>
        where O: Object<S>, S: Service {
}

#[derive(Debug)]
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
