package common

import (
	"encoding/json"
	"fmt"
	"testing"
)

func TestExample04_00(t *testing.T) {
	res1D := &Response1{
		Page:   1,
		Fruits: []string{"apple", "peach", "pear"}}
	res1B, _ := json.Marshal(res1D)
	fmt.Println(string(res1B))

	res2D := &Response2{
		Page:   1,
		Fruits: []string{"apple", "peach", "pear"}}
	res2B, _ := json.Marshal(res2D)
	fmt.Println(string(res2B))
}

func TestExample04_01(t *testing.T) {
	str := `{"page": 1, "fruits": ["apple", "peach"]}`
	res := Response2{}
	_ = json.Unmarshal([]byte(str), &res)
	fmt.Println(res)
}

type Response1 struct {
	Page   int
	Fruits []string
}
type Response2 struct {
	Page   int      `json:"page"`
	Fruits []string `json:"fruits"`
}
