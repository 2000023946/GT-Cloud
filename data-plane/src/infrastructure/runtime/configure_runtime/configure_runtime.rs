use crate::domain::job::Job;
use crate::infrastructure::runtime::state::RuntimeState;

pub struct ConfigureRuntime;

impl ConfigureRuntime {
    pub fn configure(
        &self,
        state: &mut RuntimeState,
        job: Job,
    ) -> Result<(), String> {
        todo!()
    }
}