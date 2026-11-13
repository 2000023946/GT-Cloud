import { Job } from "../domain/job/job";

export interface ControlPlane {
    submitJob(job: Job): Promise<Job>;
    getJob(jobId: string): Promise<Job>;
    listJobs(): Promise<Job[]>;
    stopJob(jobId: string): Promise<void>;
}