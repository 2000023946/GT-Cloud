package execution

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/action"

type ActionExecutor interface {
	Execute(action action.Action) error
}
