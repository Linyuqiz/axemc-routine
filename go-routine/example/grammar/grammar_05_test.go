package grammar

import (
	"strings"
	"testing"
)

func TestGrammar05_00(t *testing.T) {
	str := "This loop will run forever."
	println(str)
	println(len(str))
	println(strings.Join([]string{"a", "b", "c"}, "-"))
}

func TestGrammar05_01(t *testing.T) {
	// 一维
	var nums = []int32{1, 2, 3, 4, 5}
	for i, j := range nums {
		println(i, j)
	}
	// 二维
	var nums2 = [][]int32{
		{1, 2, 3},
		{4, 5, 6},
	}
	for i, j := range nums2 {
		for j, k := range j {
			println(i, j, k)
		}
	}
}

func TestGrammar05_02(t *testing.T) {
	content := make(map[string]string)
	content["name"] = "app"
	content["description"] = "description"
	for k, v := range content {
		println(k, v)
	}
}

func TestGrammar05_03(t *testing.T) {
	var x = 20
	var y *int = &x
	var z **int = &y
	println(x, *y, **z)

}

func TestGrammar05_04(t *testing.T) {
	app := new(app)
	app.name = "app"
	app.description = "description"
	app.display()
}

type app struct {
	name        string
	description string
}

func (a app) display() {
	println(a.name, a.description)
}
