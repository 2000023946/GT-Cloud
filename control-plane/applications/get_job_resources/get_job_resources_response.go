package get_job_resources

type GetJobResourcesResponse struct {
	CpuUsed    float32
	MemoryUsed float32
	CpuMax     float32
	MemoryMax  float32
	IsError    bool
	Message    string
}

func GetJobResourcesResponseSuccess(
	cpu_used float32, memory_used float32, cpu_max float32, memory_max float32,
) GetJobResourcesResponse {
	return GetJobResourcesResponse{
		CpuUsed: cpu_used, MemoryUsed: memory_used, CpuMax: cpu_max, MemoryMax: memory_max,
		IsError: false, Message: "",
	}
}

func GetJobResourcesResponseError(message string) GetJobResourcesResponse {
	return GetJobResourcesResponse{IsError: true, Message: message}
}
