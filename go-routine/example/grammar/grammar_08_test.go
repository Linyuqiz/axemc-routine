package grammar

import (
	"errors"
	"testing"
)

func TestGrammar08_00(t *testing.T) {
	err := errors.New("This is an error.")
	println(err.Error())
}

func TestGrammar08_01(t *testing.T) {
	//panic("This is an error.")
}
