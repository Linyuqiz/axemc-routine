package grammar

import (
	"fmt"
	"os"
	"testing"
)

func TestGrammar11_00(t *testing.T) {
	for i := range os.Environ() {
		fmt.Println(i, os.Environ()[i])
	}
}

func TestGrammar11_01(t *testing.T) {
	_ = os.Setenv("FOO", "1")
	fmt.Println("FOO:", os.Getenv("FOO"))
	fmt.Println("BAR:", os.Getenv("BAR"))
}
