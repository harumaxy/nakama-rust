pub mod google {
    pub mod api {
        include!("gen/google.api.rs");
    }
}

pub mod grpc_gateway_protoc_gen_openapiv2_options {
    include!("gen/grpc.gateway.protoc_gen_openapiv2.options.rs");
}

pub mod nakama {
    pub mod api {
        include!("gen/nakama.api.rs");
    }
    pub mod realtime {
        include!("gen/nakama.realtime.rs");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
