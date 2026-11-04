import { ControlPlane } from "../../ports/control_plane";

export class StopJobService {
    constructor(private readonly controlPlane: ControlPlane) {}

    async stop(jobId: string): Promise<void> {
        return this.controlPlane.stopJob(jobId);
    }
}