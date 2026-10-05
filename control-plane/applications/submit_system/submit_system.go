package submit_system

type SubmitSystem interface {
	Execute(request SubmitSystemRequest) error
}
