use crate::domain::job_state::job_state::JobState;

pub trait GetJobState {
    fn get_state(&self, job_id: String) -> Result<JobState, String>;
}