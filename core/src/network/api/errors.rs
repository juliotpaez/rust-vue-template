use warp::reject::Reject;

#[derive(Debug)]
pub struct ConnectionAlreadyInUse;

impl Reject for ConnectionAlreadyInUse {}

#[derive(Debug)]
pub struct OriginAddressRequired;

impl Reject for OriginAddressRequired {}