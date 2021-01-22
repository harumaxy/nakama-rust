fn main() {
    let import_paths = &[
        "nakama/apigrpc",
        "nakama/vendor",
        "nakama/build/grpc-gateway-v2.0.1/third_party/googleapis",
        "nakama/vendor/github.com/grpc-ecosystem/grpc-gateway/v2",
    ];

    let protos = &[
        "nakama/apigrpc/apigrpc.proto",
        "nakama/vendor/github.com/heroiclabs/nakama-common/api/api.proto",
        "nakama/vendor/github.com/heroiclabs/nakama-common/rtapi/realtime.proto",
    ];
    tonic_build::configure()
        .out_dir("src/gen")
        .build_server(false)
        .compile(protos, import_paths)
        .unwrap();
}
