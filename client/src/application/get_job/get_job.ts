import { Job } from "../../domain/job/job";
import { ControlPlane } from "../../ports/control_plane";

export class GetJobService {
    constructor(private readonly controlPlane: ControlPlane) {}

    async get(jobId: string): Promise<Job> {
        return this.controlPlane.getJob(jobId);
    }
}