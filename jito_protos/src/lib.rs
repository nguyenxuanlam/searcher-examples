pub mod convert;

pub mod block {
    tonic::include_proto!("block");
}

pub mod block_engine {
    tonic::include_proto!("block_engine");
}

pub mod bundle {
    tonic::include_proto!("bundle");
}

pub mod packet {
    tonic::include_proto!("packet");
}

pub mod relayer {
    tonic::include_proto!("relayer");
}

pub mod searcher {
    tonic::include_proto!("searcher");
}

pub mod shared {
    tonic::include_proto!("shared");
}

pub mod auth {
    tonic::include_proto!("auth");
}

pub mod proto {
    // Re-export all the modules
    pub use crate::auth::*;
    pub use crate::block::*;
    pub use crate::block_engine::*;
    pub use crate::bundle::*;
    pub use crate::packet::*;
    pub use crate::relayer::*;
    pub use crate::searcher::*;
    pub use crate::shared::*;
}