// @generated
#[cfg(feature = "aliasreader")]
// @@protoc_insertion_point(attribute:aliasreader)
pub mod aliasreader {
    include!("aliasreader.rs");
    // @@protoc_insertion_point(aliasreader)
}
#[cfg(feature = "appsender")]
// @@protoc_insertion_point(attribute:appsender)
pub mod appsender {
    include!("appsender.rs");
    // @@protoc_insertion_point(appsender)
}
pub mod google {
    #[cfg(feature = "google_protobuf")]
    // @@protoc_insertion_point(attribute:google.protobuf)
    pub mod protobuf {
        include!("google.protobuf.rs");
        // @@protoc_insertion_point(google.protobuf)
    }
}
#[cfg(feature = "helloworld")]
// @@protoc_insertion_point(attribute:helloworld)
pub mod helloworld {
    include!("helloworld.rs");
    // @@protoc_insertion_point(helloworld)
}
#[cfg(feature = "http_proto")]
// @@protoc_insertion_point(attribute:http_proto)
pub mod http {
    include!("http.rs");
    // @@protoc_insertion_point(http)
    #[cfg(feature = "http_responsewriter")]
    // @@protoc_insertion_point(attribute:http.responsewriter)
    pub mod responsewriter {
        include!("http.responsewriter.rs");
        // @@protoc_insertion_point(http.responsewriter)
    }
}
pub mod io {
    pub mod prometheus {
        #[cfg(feature = "io_prometheus_client")]
        // @@protoc_insertion_point(attribute:io.prometheus.client)
        pub mod client {
            include!("io.prometheus.client.rs");
            // @@protoc_insertion_point(io.prometheus.client)
        }
    }
    #[cfg(feature = "io_reader")]
    // @@protoc_insertion_point(attribute:io.reader)
    pub mod reader {
        include!("io.reader.rs");
        // @@protoc_insertion_point(io.reader)
    }
    #[cfg(feature = "io_writer")]
    // @@protoc_insertion_point(attribute:io.writer)
    pub mod writer {
        include!("io.writer.rs");
        // @@protoc_insertion_point(io.writer)
    }
}
#[cfg(feature = "keystore")]
// @@protoc_insertion_point(attribute:keystore)
pub mod keystore {
    include!("keystore.rs");
    // @@protoc_insertion_point(keystore)
}
#[cfg(feature = "messenger")]
// @@protoc_insertion_point(attribute:messenger)
pub mod messenger {
    include!("messenger.rs");
    // @@protoc_insertion_point(messenger)
}
pub mod net {
    #[cfg(feature = "net_conn")]
    // @@protoc_insertion_point(attribute:net.conn)
    pub mod conn {
        include!("net.conn.rs");
        // @@protoc_insertion_point(net.conn)
    }
}
#[cfg(feature = "p2p")]
// @@protoc_insertion_point(attribute:p2p)
pub mod p2p {
    include!("p2p.rs");
    // @@protoc_insertion_point(p2p)
}
#[cfg(feature = "plugin")]
// @@protoc_insertion_point(attribute:plugin)
pub mod plugin {
    include!("plugin.rs");
    // @@protoc_insertion_point(plugin)
}
#[cfg(feature = "rpcdb")]
// @@protoc_insertion_point(attribute:rpcdb)
pub mod rpcdb {
    include!("rpcdb.rs");
    // @@protoc_insertion_point(rpcdb)
}
#[cfg(feature = "sharedmemory")]
// @@protoc_insertion_point(attribute:sharedmemory)
pub mod sharedmemory {
    include!("sharedmemory.rs");
    // @@protoc_insertion_point(sharedmemory)
}
#[cfg(feature = "subnetlookup")]
// @@protoc_insertion_point(attribute:subnetlookup)
pub mod subnetlookup {
    include!("subnetlookup.rs");
    // @@protoc_insertion_point(subnetlookup)
}
#[cfg(feature = "validatorstate")]
// @@protoc_insertion_point(attribute:validatorstate)
pub mod validatorstate {
    include!("validatorstate.rs");
    // @@protoc_insertion_point(validatorstate)
}
#[cfg(feature = "vm")]
// @@protoc_insertion_point(attribute:vm)
pub mod vm {
    include!("vm.rs");
    // @@protoc_insertion_point(vm)
}
