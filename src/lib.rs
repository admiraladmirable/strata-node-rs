// Include the `items` module, which is generated from items.proto.
// It is important to maintain the same structure as in the proto.
pub mod quivr {
    pub mod models {
        include!(concat!(env!("OUT_DIR"), "/quivr.models.rs"));
    }
}

pub mod grpc {
    pub mod health {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/grpc.health.v1.rs"));
        }
    }
}

pub mod co {
    pub mod topl {
        pub mod proto {
            pub mod node {
                include!(concat!(env!("OUT_DIR"), "/co.topl.proto.node.rs"));
            }
        }
        pub mod node {
            pub mod models {
                include!(concat!(env!("OUT_DIR"), "/co.topl.node.models.rs"));
            }
            pub mod services {
                include!(concat!(env!("OUT_DIR"), "/co.topl.node.services.rs"));
            }
        }
        pub mod genus {
            pub mod services {
                include!(concat!(env!("OUT_DIR"), "/co.topl.genus.services.rs"));
            }
        }
        pub mod consensus {
            pub mod models {
                include!(concat!(env!("OUT_DIR"), "/co.topl.consensus.models.rs"));
            }
        }
        pub mod brambl {
            pub mod models {
                pub mod common {
                    include!(concat!(env!("OUT_DIR"), "/co.topl.brambl.models.common.rs"));
                }
                pub mod r#box {
                    // box is a reserved keyword!
                    include!(concat!(env!("OUT_DIR"), "/co.topl.brambl.models.r#box.rs"));
                }
                pub mod transaction {
                    include!(concat!(
                        env!("OUT_DIR"),
                        "/co.topl.brambl.models.transaction.rs"
                    ));
                }

                include!(concat!(env!("OUT_DIR"), "/co.topl.brambl.models.rs"));
            }
        }
    }
}
