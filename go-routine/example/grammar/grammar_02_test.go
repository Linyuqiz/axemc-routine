package grammar

import (
	"fmt"
	"testing"
	"time"
)

func TestGrammar02_00(t *testing.T) {
	str := "Hello World"
	if str[0] == 'H' {
		fmt.Println("Hello World")
	}
}

func TestGrammar02_01(t *testing.T) {
	str := "Hello World"
	if str[0] == 'H' {
		fmt.Println("the word starts with H")
	} else {
		fmt.Println("the word doesn't start with H")
	}
}

func TestGrammar02_02(t *testing.T) {
	str := "Hello World"
	switch str {
	case "Hello World":
		fmt.Println("the word is Hello World")
	case "Hello":
		fmt.Println("the word is Hello")
	case "World":
		fmt.Println("the word is World")
	default:
		fmt.Println("the word is unknown")
	}
}

func TestGrammar02_03(t *testing.T) {
	chans := make([]chan int, 3)
	select {
	case x := <-chans[0]:
		fmt.Println("the number is", x)
	case y := <-chans[1]:
		fmt.Println("the number is", y)
	case z := <-chans[2]:
		fmt.Println("the number is", z)
	default:
		fmt.Println("the number is unknown")
	}
}

func TestGrammar02_04(t *testing.T) {
	c1 := make(chan string, 1)
	go func() {
		time.Sleep(time.Second * 2)
		c1 <- "result message"
	}()
	select {
	case res := <-c1:
		fmt.Println(res)
	//case <-time.After(time.Second * 1):
	//	fmt.Println("timeout message 1s")
	case <-time.After(time.Second * 3):
		fmt.Println("timeout message 3s")
	}
}

func TestGrammar02_05(t *testing.T) {
	c := make(chan bool)
	go func() {
		for {
			select {
			case <-c:
				println("status: ", true)
				return
			default:
				println("---")
			}
		}
	}()
	c <- true
}
