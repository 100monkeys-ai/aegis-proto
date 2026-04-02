pub mod aegis {
    pub mod runtime {
        pub mod v1 {
            tonic::include_proto!("aegis.runtime.v1");
        }
    }
    pub mod seal_gateway {
        pub mod v1 {
            tonic::include_proto!("aegis.seal_gateway.v1");
        }
    }
    pub mod cluster {
        pub mod v1 {
            tonic::include_proto!("aegis.cluster.v1");
        }
    }
    pub mod storage {
        pub mod v1 {
            tonic::include_proto!("aegis.storage.v1");
        }
    }
    pub mod cortex {
        pub mod v1 {
            tonic::include_proto!("aegis.cortex.v1");
        }
    }
}

pub mod embedding {
    tonic::include_proto!("embedding");
}
