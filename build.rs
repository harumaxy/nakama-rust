fn main() {
    tonic_build::configure()
        .compile(
            &[
                "nakama/apigrpc/apigrpc.proto",
                "nakama/vendor/github.com/heroiclabs/nakama-common/api/api.proto",
                "nakama/vendor/github.com/heroiclabs/nakama-common/rtapi/realtime.proto",
            ],
            &[
                "nakama/apigrpc",
                "nakama/vendor",
                "nakama/build/grpc-gateway-v2.0.1/third_party/googleapis",
                "nakama/vendor/github.com/grpc-ecosystem/grpc-gateway/v2",
            ],
        )
        .map_err(|e| println!("{}", e))
        .unwrap();
}
