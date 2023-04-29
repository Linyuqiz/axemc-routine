package grammar

import (
	"go/types"
	"testing"
	"unsafe"
)

func TestGrammar14_00(t *testing.T) {
	// 内置函数new 计算类型大小，为其分配零值内存，返回指针
	n := new(types.Nil)
	println(n)
	println(unsafe.Sizeof(n))

	// 而make会被编译器翻译成具体的创建函数，由其分配内存和初始化成员结构，返回对象而非指针
	ints := make([]int, 0)
	println(ints)
	println(unsafe.Sizeof(ints))
}
