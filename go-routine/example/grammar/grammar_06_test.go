package grammar

import (
	"testing"
)

func TestGrammar06_00(t *testing.T) {
	println(factorial00(5))
}

func factorial00(x uint) uint {
	if x == 0 {
		return 1
	}
	return x * factorial00(x-1)
}
