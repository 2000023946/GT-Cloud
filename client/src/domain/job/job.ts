import { App } from "../app/app";
import { JobStatus } from "./job_status";
import { NetworkRule } from "../network/network_rule";
import { Resource } from "../resource/resource";

export interface Job {
    id: string;
    app: App;
    status: JobStatus;
    resources: Resource;
    network: NetworkRule[];
}