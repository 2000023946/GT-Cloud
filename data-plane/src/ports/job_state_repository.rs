use crate::domain::job_state::job_state::JobState;

pub trait JobStateRepository {
    fn get(&self, job_id: &str) -> Result<JobState, String>;

    fn save(&self, state: JobState) -> Result<(), String>;

    fn get_all_job_ids(&self) -> Result<Vec<String>, String>;
}