package application

type IJsonAdapter interface {
	Marshal(content any) ([]byte, error)
	Unmarshal(data []byte, v any) error
}
