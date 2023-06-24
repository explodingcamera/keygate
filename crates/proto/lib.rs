pub mod v1 {
    pub mod api {
        pub mod identity {
            include!(concat!(env!("OUT_DIR"), "/v1.api.identity.rs"));
        }

        pub mod login {
            include!(concat!(env!("OUT_DIR"), "/v1.api.login.rs"));
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
