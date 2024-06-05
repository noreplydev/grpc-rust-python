pub mod entity {
    tonic::include_proto!("entity"); // the parameter here is the package name specified in your .proto file
}

use entity::entity_provider_server::{EntityProvider, EntityProviderServer};
use entity::{Entity, EntityType};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
struct MyEntityProvider;

#[tonic::async_trait]
impl EntityProvider for MyEntityProvider {
    async fn generate_entity(
        &self,
        request: Request<EntityType>,
    ) -> Result<Response<Entity>, Status> {
        Ok(Response::new(Entity {
            data: "Sample data".to_string(),
            r#type: request.into_inner().r#type,
            index: 0,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let entity_provider = MyEntityProvider::default();

    println!("EntityProviderServer listening on {}", addr);
    Server::builder()
        .add_service(EntityProviderServer::new(entity_provider))
        .serve(addr)
        .await?;

    Ok(())
}
