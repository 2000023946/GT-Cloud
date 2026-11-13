import { Job } from "../../domain/job/job";
import { ControlPlane } from "../../ports/control_plane";

export class ListJobsService {
    constructor(private readonly controlPlane: ControlPlane) {}

    async list(): Promise<Job[]> {
        return this.controlPlane.listJobs();
    }
}