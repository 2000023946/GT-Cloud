package repository

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/user"

type UserRepository interface {
	Create(user user.User) error
	Get(id string) (user.User, error)
	GetAll() ([]user.User, error)
	Update(user user.User) error
	Delete(id string) error
}
