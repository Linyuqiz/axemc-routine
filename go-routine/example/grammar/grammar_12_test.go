package grammar

import (
	"testing"
)

func TestGrammar12_00(t *testing.T) {
	var c chan bool
	// 如果给一个nil 的channel 发送数据，会造成永远阻塞
	c <- true
}

func TestGrammar12_01(t *testing.T) {
	var c chan bool
	// 如果给一个nil 的channel 发送数据，会造成永远阻塞
	<-c
}

func TestGrammar12_02(t *testing.T) {
	var c chan bool
	// close chan
	close(c)
	// 给一个已经关闭的channel 发送数据，会引起panic
	c <- true
}

func TestGrammar12_03(t *testing.T) {
	var c chan bool
	// close chan
	close(c)
	// 从一个已经关闭的channel 接收数据，如果缓冲区中为空，则返回一个零值
	<-c
}
