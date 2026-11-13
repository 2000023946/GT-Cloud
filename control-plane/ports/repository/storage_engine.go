package repository

type StorageEngine interface {
	Get(key string) ([]byte, error)
	Put(key string, value []byte) error
	Delete(key string) error
}
