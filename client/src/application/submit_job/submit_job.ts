import { Job } from "../../domain/job/job";
import { ControlPlane } from "../../ports/control_plane";

export class SubmitJobService {
    constructor(private readonly controlPlane: ControlPlane) {}

    async submit(job: Job): Promise<Job> {
        return this.controlPlane.submitJob(job);
    }
}