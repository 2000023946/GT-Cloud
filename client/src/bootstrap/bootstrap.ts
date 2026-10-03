import { API } from "../api/api";

import { GetJobService } from "../application/get_job/get_job";
import { ListJobsService } from "../application/list_jobs/list_jobs";
import { StopJobService } from "../application/stop_job/stop_job";
import { SubmitJobService } from "../application/submit_job/submit_job";
import { HttpControlPlane } from "../infrastructure/http_control_plane";


export function start(): API {
    const controlPlane = new HttpControlPlane();

    const submitJobService = new SubmitJobService(
        controlPlane,
    );

    const getJobService = new GetJobService(
        controlPlane,
    );

    const listJobsService = new ListJobsService(
        controlPlane,
    );

    const stopJobService = new StopJobService(
        controlPlane,
    );

    return new API(
        submitJobService,
        getJobService,
        listJobsService,
        stopJobService,
    );
}