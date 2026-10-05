package repository

import "github.gatech.edu/mabucar3/GT-Cloud/control-plane/domain/replica"

type ReplicaRepository interface {
	Create(replica replica.Replica) error
	Get(id string) (replica.Replica, error)
	GetAll() ([]replica.Replica, error)
	Update(replica replica.Replica) error
	Delete(id string) error
}
