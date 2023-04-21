use super::Backend;
use crate::protos::university::{self, *};

#[tonic::async_trait]
impl university_service_server::UniversityService for Backend {
    async fn register_new_university(
        &self,
        request: tonic::Request<NewUniversityParam>,
    ) -> Result<tonic::Response<UniversityCreationResponse>, tonic::Status> {
        todo!()
    }

    async fn get_university_info(
        &self,
        request: tonic::Request<GetUniversityParam>,
    ) -> Result<tonic::Response<UniversityInfoResponse>, tonic::Status> {
        let request = request.into_inner();

        let university_id = request.university_id;
        
        let 

        todo!()
    }
}
