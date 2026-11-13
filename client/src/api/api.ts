import { Job } from "../domain/job/job";
import { GetJobService } from "../application/get_job/get_job";
import { ListJobsService } from "../application/list_jobs/list_jobs";
import { StopJobService } from "../application/stop_job/stop_job";
import { SubmitJobService } from "../application/submit_job/submit_job";

export class API {
    constructor(
        private readonly submitJobService: SubmitJobService,
        private readonly getJobService: GetJobService,
        private readonly listJobsService: ListJobsService,
        private readonly stopJobService: StopJobService,
    ) {}

    async submitJob(job: Job): Promise<Job> {
        console.log("API: SubmitJob");
        return this.submitJobService.submit(job);
    }

    async getJob(jobId: string): Promise<Job> {
        console.log("API: GetJob");
        return this.getJobService.get(jobId);
    }

    async listJobs(): Promise<Job[]> {
        console.log("API: ListJobs");
        return this.listJobsService.list();
    }

    async stopJob(jobId: string): Promise<void> {
        console.log("API: StopJob");
        return this.stopJobService.stop(jobId);
    }
}