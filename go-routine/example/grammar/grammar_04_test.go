package grammar

import (
	"math/rand"
	"testing"
	"time"
)

func TestGrammar04_00(t *testing.T) {
	println(random00())
}

func TestGrammar04_01(t *testing.T) {
	x, y, z := random00()
	println(x, y, z)
	swap00(&x, &y)
	println(x, y, z)
}

func TestGrammar04_02(t *testing.T) {
	getRandom := func() int {
		rand.Seed(time.Now().UnixNano())
		return rand.Intn(50)
	}
	println(getRandom())
}

func random00() (int, int, int) {
	rand.Seed(time.Now().UnixNano())
	return rand.Intn(50), rand.Intn(100), rand.Intn(150)
}

func swap00(x *int, y *int) {
	*x, *y = *y, *x
}
