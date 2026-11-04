package update_system

type UpdateSystem interface {
	Execute(request UpdateSystemRequest) error
}
