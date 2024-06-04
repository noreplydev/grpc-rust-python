pub mod entity {
    tonic::include_proto!("entity"); // the parameter here is the package name specified in your .proto file
}

use entity::entity_provider_server::{EntityProvider, EntityProviderServer};
use entity::{Entity, EntityType};
use tonic::{Request, Response, Status};

struct MyEntityProvider;

#[tonic::async_trait]
impl EntityProvider for MyEntityProvider {
    async fn generate_entity(
        &self,
        request: Request<EntityType>,
    ) -> Result<Response<Entity>, Status> {
        Ok(Response::new(Entity {
            data: "Hello".to_string(),
            r#type: "Hello".to_string(),
            index: 0,
        }))
    }
}

fn main() {}
