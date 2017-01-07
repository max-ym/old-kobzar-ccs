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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
