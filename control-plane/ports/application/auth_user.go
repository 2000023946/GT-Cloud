package application

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/user"

type AuthUser interface {
	GetAuthenticatedUser() (user.User, error)
}
