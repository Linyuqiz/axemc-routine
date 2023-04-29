package grammar

import (
	"sort"
	"testing"
)

func TestGrammar13_00(t *testing.T) {
	l := make([]int, 0, 10)
	println(len(l))
	println(cap(l))
}

func TestGrammar13_01(t *testing.T) {
	type App struct {
		Name string
	}
	app1 := new(App)
	println(app1)
	app2 := &App{}
	println(app2)
}

func TestGrammar13_02(t *testing.T) {
	a1 := make([]int, 0)
	m := make(map[int]int)
	for i := 0; i < 10000; i++ {
		a1 = append(a1, i)
		m[cap(a1)] = 1
	}

	a2 := make([]int, 0)
	for i := range m {
		a2 = append(a2, i)
	}

	sort.Ints(a2)
	// 1024之前，扩容为原来的一倍；1024之后，就变成了每次都是扩容了四分之一左右
	for _, v := range a2 {
		println(v)
	}
}
