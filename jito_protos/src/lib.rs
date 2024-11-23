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
    tonic::include_proto!("packet");
    tonic::include_proto!("block");
    tonic::include_proto!("block_engine");
    tonic::include_proto!("bundle");
    tonic::include_proto!("relayer");
    tonic::include_proto!("searcher");
    tonic::include_proto!("shared");
    tonic::include_proto!("auth");
    // Re-export shared types to avoid conflicts
    pub use packet::{SubscribePacketsRequest, SubscribePacketsResponse};

}