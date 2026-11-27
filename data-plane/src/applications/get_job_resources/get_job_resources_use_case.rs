use crate::{applications::get_job_resources::{get_job_resources_request::GetJobResourcesRequest, get_job_resources_response::GetJobResourcesResponse}, domain::resource::Resource, ports::resource_manager::ResourceManager};

pub struct GetJobResourcesUseCase<R: ResourceManager> {
    pub resource_manager: R
}

impl<R: ResourceManager>  GetJobResourcesUseCase<R> {
    // call monitor on the resource get the current resource
    // return success or failure based on that 
    pub fn execute(&self, request: GetJobResourcesRequest) -> GetJobResourcesResponse {
        let resource =  match self.resource_manager.monitor(request.job_id.clone()) {
            Ok(resource) => resource,
            Err(err) => {
                return GetJobResourcesResponse::error(err)
            }
        };
        return GetJobResourcesResponse::success(resource.cpu_usage, resource.memory_usage);
    }
}

