use proto::protos::university::university_service_server::UniversityServiceServer;
use proto::tonic;
use proto::tonic_reflection;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_root_address = "127.0.0.1:8080".parse().unwrap();
    let generic_backend = proto::endpoints::Backend::default();

    eprintln!("Listining on: {server_root_address}");

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::protos::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    let university_service = UniversityServiceServer::new(generic_backend);
    Server::builder()
        .add_service(university_service)
        .add_service(reflection_service)
        .serve(server_root_address)
        .await?;

    Ok(())
}
