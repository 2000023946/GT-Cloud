package json_adapter

import "encoding/json"

type JsonAdapter struct {
}

func (s *JsonAdapter) Marshal(content any) ([]byte, error) {
	return json.Marshal(content)
}

func (s *JsonAdapter) Unmarshal(data []byte, v any) error {
	return json.Unmarshal(data, v)
}
