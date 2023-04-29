package grammar

import (
	"fmt"
	"testing"
)

func TestGrammar09_00(t *testing.T) {
	outputContent := func() {
		fmt.Println("Hello World")
	}
	go outputContent()
}

func TestGrammar09_01(t *testing.T) {
	messages := make(chan string, 2)
	getMessage := func(messages chan string) {
		messages <- "the massage from other channel"
	}

	go getMessage(messages)
	messages <- "the massage from other channel1"
	println(<-messages)
	println(<-messages)
}

func TestGrammar09_02(t *testing.T) {
	ping := func(pings chan<- string, msg string) {
		pings <- msg
	}
	pong := func(pings <-chan string, pongs chan<- string) {
		//msg := <-pings
		//pongs <- msg
		pongs <- <-pings
	}

	pings := make(chan string, 1)
	pongs := make(chan string, 1)
	ping(pings, "passed message")
	pong(pings, pongs)
	fmt.Println(<-pongs)
}
