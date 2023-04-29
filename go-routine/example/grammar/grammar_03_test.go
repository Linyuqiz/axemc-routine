package grammar

import (
	"fmt"
	"testing"
)

func TestGrammar03_00(t *testing.T) {
	for true {
		fmt.Printf("This loop will run forever.\n")
	}
}

func TestGrammar03_01(t *testing.T) {
	for i := 0; i < 10; i++ {
		fmt.Printf("The value of i is %d.\n", i)
	}
}

func TestGrammar03_02(t *testing.T) {
	for i := 0; i < 10; i++ {
		if i%2 == 0 {
			continue
		}
		if i > 5 {
			break
		}
		fmt.Printf("The value of i is %d.\n", i)
	}
}

func TestGrammar03_03(t *testing.T) {
LOOP:
	for i := 0; i < 10; i++ {
		if i%2 == 0 {
			continue
		}
		if i > 5 {
			goto LOOP
		}
		fmt.Printf("The value of i is %d.\n", i)
	}
}
