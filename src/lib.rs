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
    fn myself<OO: OwnedObject<S, Id = Self::Id>>() -> OO;

    /// The object that called this function quits.
    /// All allocated resources are freed. All services registered
    /// by the object are removed. All sub-objects are killed.
    fn decease() -> !;

    /// Get a CCS Network reference for this object.
    fn network<N: Network<S>>(&self) -> &N;
}

pub trait OwnedObject<S>: Object<S> where S: Service {

    /// Kill given owned object. All resources local for this object
    /// are released. All services provided by this object are discarded.
    /// Fail when object is not alive. The function consumes the
    /// object and even on error may not give ownership back (when
    /// object is no longer alive).
    fn kill(self) -> Result<(), ObjectKillErr>;

    /// Check if given object is still alive. It is alive if
    /// main thread is running or at least one service is provided.
    fn is_alive(&self) -> bool;

    /// Get a CCS Network reference that exists inside the object.
    /// That is, all sub-objects and their services are created in
    /// master-object's internal network. It is not visible from the
    /// outside of that object in its external network.
    fn internal_network<ON: OpenNetwork<S>>(&self) -> &ON;

    /// Get an external CCS Network reference for this object.
    fn network<ON: OpenNetwork<S>>(&self) -> &ON;
}

/// Errors that appear on failed attempt to kill an object.
pub enum ObjectKillErr {

    /// Object is not alive and cannot be killed.
    NotAlive,
}

/// A CCS network.
pub trait Network<S: Service>: Sized {

}

/// A CCS network that is open for current object. Current object
/// can register new services or request them in its open networks.
pub trait OpenNetwork<S>: Network<S> where S: Service {

    /// Connect to a service provider. If any object in CCS network can
    /// provide such service, then channel is created.
    fn connect<O, SC>(&self, service: S) -> Result<SC, S>
        where O     : Object<S>,
              SC    : Socket<O, S>;

    /// Attempt to register new service that current object is ready to
    /// provide.
    fn register<O, OS, SC>(&self, reg_form: RegistrationForm<O, S, SC>)
        -> Result<OS, RegistrationErr>
        where   O   : Object<S>,
                OS  : OwnedService<Id = S::Id>,
                SC  : Socket<O, S>;

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
        (&self, reg_form: RegistrationForm<O, S, SC>)
        -> Result<OS, RegistrationErr>
        where   O   : Object<S>,
                OS  : OwnedService<Id = S::Id>,
                SC  : Socket<O, S>;
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

    /// Make a service pointer with given identifier.
    /// This function always succeedes even if no object in the system
    /// provides service with given identifier.
    /// When this service will be requested, if no object is providing
    /// the service, channel would not be established.
    /// The objects that provide this service can be created or deceased
    /// after this pointer is created and so even if channel was
    /// established once, it does not guaranteed that channels will
    /// always succeed to be established later in time.
    fn by_id(id: Self::Id) -> Self;
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
                SC  : Socket<O, Self>;
}

pub struct RegistrationForm<O, S, SC>
        where O     : Object<S>,
              S     : Service,
              SC    : Socket<O, S>
{
    _a      : std::marker::PhantomData<O>,

    /// Entry point. When service is requested, execution starts
    /// from given function.
    pub entry   : fn(SC) -> !,

    /// Identifier of the service.
    pub id      : S::Id,
}

impl<O, S, SC> RegistrationForm<O, S, SC>
        where O     : Object<S>,
              S     : Service,
              SC    : Socket<O, S>
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
        entry   : fn(SC) -> !,
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
/// implementation of the Socket trait for channel.
pub trait Socket<O, S>: Sized
        where O: Object<S>, S: Service {

    /// Get object which requested the service.
    fn requester(&self) -> &O;

    /// Get service that was requested.
    fn service(&self) -> &S;
    
    /// Wait forever until some data is received or socket error occurs.
    fn receive(&self) -> Result<&Data, SocketErr>;
    
    /// Wait for given amount of time to receive a data from the service
    /// provider. Similar to 'receive' function. After timeout, None will
    /// be returned.
    fn wait_to_receive(&self, time: Time) -> Option<Result<&Data, SocketErr>>;
    
    /// Wait forever until requester receives the data or socket error
    /// occurs.
    fn send(&self, data: Data) -> Result<(), SocketErr>;
    
    /// Wait for given amount of time to send a data to the service requester.
    /// Similar to 'send' function. After timeout, None will
    /// be returned.
    fn wait_to_send(&self, time: Time) -> Option<Result<(), SocketErr>>;
    
    /// Close the socket and the channel.
    fn close(self);
    
    /// Run some function that can be safely aborted when channel gets closed.
    fn run_abortable(&self, run_fn: Fn()) -> AbortResult;
    
    /// Check if channel still is opened.
    fn check(self) -> Option<Self>;
    
    /// Check if channel is opened. Similar to 'check', but does
    /// not consume the socket and returns boolean value instead.
    fn is_opened(&self) -> bool;
}

/// Some data that is transfered via channels.
pub trait Data {
}

/// The time. Used in timers.
pub trait Time {
    // TODO
}

/// Error that appears in operation with socket.
#[derive(Debug)]
pub enum SocketErr {
    
    /// Operation cannot be performed because channel is closed.
    ChannelClosed,
    
    /// Operation was canceled because two sockets tried the same operation
    /// in the same time (like two receives or two sends). This
    /// error was created so that two objects didn't get into the locked state.
    /// Error is received only by the last socket which tried to perform
    /// the operation.
    Lockup,
}

/// Result of running the function that could get aborted if channel closes.
#[derive(Debug)]
pub enum AbortResult {
    
    /// Function was aborted because channel was closed.
    Aborted,
    
    /// Function execution finished and channel was not yet closed.
    Finished
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
