package json_adapter

import "encoding/json"

type JsonAdapter struct {
}

func (s *JsonAdapter) Marshal(content any) ([]byte, error) {
	return json.Marshal(content)
}
