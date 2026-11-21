package application

type IJsonAdapter interface {
	Marshal(content any) ([]byte, error)
}
