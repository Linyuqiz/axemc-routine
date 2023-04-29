package grammar

import (
	"bytes"
	"fmt"
	"strings"
	"testing"
)

// byte - 与uint8相同
// rune - 与int32相同
// uint - 32或64位
// int - 与uint大小相同
// uintptr - 无符号整数，用于存储指针值的未解释位
func TestGrammar01_00(t *testing.T) {
	var a byte = 'a'
	var b rune = 'b'
	var c uint = 1
	var d int = 2
	var e uintptr = 3
	fmt.Println(a, b, c, d, e)
}

func TestGrammar01_01(t *testing.T) {
	// 性能最差
	// 每次运算都会产生一个新的字符串
	str0 := "hello" + ", world"
	println(str0)

	// 可能较常用
	// 内部的逻辑比较复杂，有很多额外的判断
	str1 := fmt.Sprintf("hello, %s", "world")
	println(str1)

	// 性能较优
	// join会先根据字符串数组的内容，计算出一个拼接之后的长度，然后申请对应大小的内存，一个一个字符串填入
	// 在已有一个数组的情况下，这种效率会很高，但是本来没有，去构造这个数据的代价也不小
	str2 := strings.Join([]string{"hello", ", ", "world"}, "")
	println(str2)

	// 性能最好
	// 当成可变字符使用，对内存的增长也有优化，如果能预估字符串的长度，还可以用 buffer.Grow() 接口来设置 capacity
	str3 := bytes.Buffer{}
	str3.Grow(6) // 初始化 capacity
	str3.WriteString("hello")
	str3.WriteString(", ")
	str3.WriteString("world")
	println(str3.String())

}
