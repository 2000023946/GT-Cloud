use crate::{applications::get_job_logs::{get_job_logs_request::GetJobLogsRequest, get_job_logs_response::GetJobLogsResponse}, infrastructure::runtime::helpers::working_directory_manager::WorkingDirectoryManager, ports::i_logs_reader::ILogsReader};

pub struct GetJobLogsUseCase<R: ILogsReader> {
    pub directory_manager: WorkingDirectoryManager,
    pub logs_reader: R
}

impl<R: ILogsReader> GetJobLogsUseCase<R> {
    pub fn new(directory_manager: WorkingDirectoryManager, logs_reader: R) -> Self{
        return Self { directory_manager, logs_reader }
    }
    pub fn execute(&self, request: GetJobLogsRequest) -> GetJobLogsResponse {
        // get the path fro mthe directory manager
        let directory = match self.directory_manager.create_directory(&request.job_id) {
                Ok(dir) => dir,
                Err(err) => {
                    return GetJobLogsResponse::error(err.to_string());
                }
            };
        // read the file using the logs reader 
        let std_err = match self.logs_reader.read_stderr(directory.clone()) {
            Ok(std_err) => std_err,
            Err(err) => {
                return GetJobLogsResponse::error(err.to_string());
            }
        };
         let std_out = match self.logs_reader.read_stdout(directory.clone()) {
            Ok(std_out) => std_out,
            Err(err) => {
                return GetJobLogsResponse::error(err.to_string());
            }
        };
        // return the response and for each operation check for errors 
        return GetJobLogsResponse::success(std_out, std_err);
    }
}