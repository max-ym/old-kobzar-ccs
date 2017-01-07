/// Holds basic functions for creating channels, services, object etc. in
/// the system. Each CCS model implementation has it's own Master. Thus,
/// each OS architecture implement their specific Master.
pub trait Master {
}

/// Object is sort of process in Kobzar. It is an instanse of some
/// program that is currently running on the system, or residing in
/// the RAM. The CCS object may request services of other objects which
/// either already are loaded or can be loaded to reply on the request.
pub trait Object {

    /// Use to separate different objects in the system so that
    /// it was possible to definitely recognize one object
    /// among all others.
    type Id;

    /// Get object identifier. Each object in one CCS network has
    /// unique identifier.
    fn id(&self) -> Id;
}

/// Service is requested by the Object. Service is used to update some
/// data, create or delete it, make some calculations or make any other
/// change to the system. It can be provided by a single program on the
/// system or by a group of different programs. CCS decides which
/// program may serve the request or may decline contributing to
/// request transferer. This trait identifies single service that can
/// be provided by any program in the system.
pub trait Service {

    /// Use to separate different services in the network so that
    /// it was possible to definitely recognize one service
    /// among all others.
    type Id;

    /// Get service identifier. Each service in one CCS network has
    /// unique identifier.
    fn id(&self) -> Id;
}

/// Channel is a conection of the requester-object that requests the
/// service and the provider object. Data transfer is performed by
/// implementation of this trait.
pub trait Channel {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
