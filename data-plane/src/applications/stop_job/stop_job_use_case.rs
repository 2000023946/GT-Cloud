use crate::{applications::stop_job::{stop_job_request::StopJobRequest, stop_job_response::StopJobResponse}, ports::{resource_manager::ResourceManager, runtime::Runtime}};

pub struct StopJobUseCase<R: Runtime, RM: ResourceManager> {
    pub runtime_manager: R,
    pub resource_manager: RM
}

impl<R: Runtime, RM: ResourceManager> StopJobUseCase<R, RM> {
    pub fn execute(&self, request: StopJobRequest) -> StopJobResponse {
        match self.runtime_manager.stop(request.job_id.clone()) {
            Ok(_) => {},
            Err(err) => {
                return StopJobResponse::error(err.to_string());
            }
        };
        match self.runtime_manager.cleanup(request.job_id.clone(

        )) {
            Ok(_) => {},
            Err(err) => {
                return StopJobResponse::error(err.to_string());
            }
        };

        match self.resource_manager.stop(request.job_id.clone()) {
            Ok(_) => {},
            Err(err) => {
                return StopJobResponse::error(err.to_string());
            }
        };

        match self.resource_manager.cleanup(request.job_id.clone()) {
            Ok(_) => {},
            Err(err) => {
                return StopJobResponse::error(err.to_string());
            }
        };
        return StopJobResponse::success();
    }
}