package grammar

import (
	"fmt"
	"os"
	"testing"
)

func setup() {
	fmt.Println("setup executed")
}

func teardown() {
	fmt.Println("teardown executed")
}

// 如果测试文件中包含函数 TestMain，那么生成的测试将调用 TestMain(m)，而不是直接运行测试。
// 调用 m.Run() 触发所有测试用例的执行，并使用 os.Exit() 处理返回的状态码，如果不为0，说明有用例失败。
// 因此可以在调用 m.Run() 前后做一些额外的准备(setup)和回收(teardown)工作
func TestMain(m *testing.M) {
	setup()
	code := m.Run()
	teardown()
	os.Exit(code)
}

func TestGrammar00_00(t *testing.T) {
	fmt.Println("Hello World")
}

func TestGrammar00_01(t *testing.T) {
	s := fmt.Sprintf("Hello %v!", "Go")
	println(s)
}

func TestGrammar00_02(t *testing.T) {
	n, err := fmt.Fprintf(os.Stdout, "Hello %v!\n", "Go")
	if err != nil {
		panic(err)
	}
	println(n)
}
