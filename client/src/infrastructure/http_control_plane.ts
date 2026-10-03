import { Job } from "../domain/job/job";
import { ControlPlane } from "../ports/control_plane";

export class HttpControlPlane implements ControlPlane {
    async submitJob(job: Job): Promise<Job> {
        console.log("ControlPlane: SubmitJob", job.id);
        return job;
    }

    async getJob(jobId: string): Promise<Job> {
        console.log("ControlPlane: GetJob", jobId);

        throw new Error("Not implemented");
    }

    async listJobs(): Promise<Job[]> {
        console.log("ControlPlane: ListJobs");

        return [];
    }

    async stopJob(jobId: string): Promise<void> {
        console.log("ControlPlane: StopJob", jobId);
    }
}