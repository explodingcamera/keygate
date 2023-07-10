pub use prost::Message;

pub mod v1 {
    pub mod api {
        pub mod identity {
            include!(concat!(env!("OUT_DIR"), "/v1.api.identity.rs"));
        }

        pub mod auth {
            include!(concat!(env!("OUT_DIR"), "/v1.api.auth.rs"));
        }

        pub mod util {
            include!(concat!(env!("OUT_DIR"), "/v1.api.util.rs"));
        }
    }

    pub mod models {
        include!(concat!(env!("OUT_DIR"), "/v1.models.rs"));

        // pub mod internal {
        //     include!(concat!(env!("OUT_DIR"), "/v1.models.internal.rs"));
        // }
    }
}

pub use v1::*;
