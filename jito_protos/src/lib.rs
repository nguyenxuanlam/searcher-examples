pub mod convert;

// Define the individual proto modules
mod block {
    tonic::include_proto!("block");
}

mod block_engine {
    tonic::include_proto!("block_engine");
}

mod bundle {
    tonic::include_proto!("bundle");
}

mod packet {
    tonic::include_proto!("packet");
}

mod relayer {
    tonic::include_proto!("relayer");
}

mod searcher {
    tonic::include_proto!("searcher");
}

mod shared {
    tonic::include_proto!("shared");
}

mod auth {
    tonic::include_proto!("auth");
}

// Public proto module that re-exports everything
pub mod proto {
    pub mod packet {
        pub use crate::packet::*;
    }
    
    pub mod block {
        pub use crate::block::*;
    }
    
    pub mod block_engine {
        pub use crate::block_engine::*;
    }
    
    pub mod bundle {
        pub use crate::bundle::*;
    }
    
    pub mod relayer {
        pub use crate::relayer::*;
    }
    
    pub mod searcher {
        pub use crate::searcher::*;
    }
    
    pub mod shared {
        pub use crate::shared::*;
    }
    
    pub mod auth {
        pub use crate::auth::*;
    }
}