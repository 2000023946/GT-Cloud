use crate::applications::start_job::start_job_response::StartJobResponse;
use crate::applications::start_job::start_job_request::StartJobRequest;
use crate::domain::job::Job;
use crate::ports::i_code_repository::ICodeRepository;
use crate::ports::resource_manager::ResourceManager;
use crate::ports::runtime::Runtime;

pub struct StartJobUseCase<RM: ResourceManager, R: Runtime, C:ICodeRepository> {
    pub resource_manager: RM,
    pub runtime_manager: R,
    pub code_repository: C
}

impl<RM: ResourceManager, R: Runtime, C: ICodeRepository> StartJobUseCase<RM, R, C> {
    pub fn new(
        resource_manager: RM,
        runtime_manager: R,
        code_repository: C
    ) -> Self {
        Self{resource_manager, runtime_manager, code_repository}
    }

    pub fn start_job(&self, request: StartJobRequest) -> StartJobResponse {
        // create the job
        let job = Job::new(
            request.id, request.name, request.cpu, request.memory, request.path.clone(),request.command
        );
        // save the code on the system
        match self.code_repository.create(request.file, request.path.clone()) {
            Err(error) => {
                return StartJobResponse::create_error(error);
            }
            Ok(_) => {},
        }
        // configure the resource and runtime
        match self.resource_manager.configure(&job) {
            Err(error) => {
                return StartJobResponse::create_error(error);
            }
            Ok(_) => {},
        }
        // configure and start the runtime
        match self.runtime_manager.configure(&job) {
            Err(error) => {
                return StartJobResponse::create_error(error);
            }
            Ok(_) => {},
        }
        match self.runtime_manager.start(&job) {
            Err(error) => {
                return StartJobResponse::create_error(error);
            }
            Ok(_) => {},
        }
        // get the pid from the runtime manager
        let pid = match self.runtime_manager.get_pid(&job) {
            Err(error) => {
                return StartJobResponse::create_error(error);
            }
            Ok(pid) => pid,
        };
        // start the resource manager 
        match self.resource_manager.start(&job, pid) {
            Err(error) => {
                return StartJobResponse::create_error(error);
            }
            Ok(_) => {},
        }
        // send success
        StartJobResponse::create_success("Job successfully started!".to_owned())
    }   
}

