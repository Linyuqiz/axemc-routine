package common

import (
	"fmt"
	"strconv"
	"testing"
)

func TestExample05_00(t *testing.T) {
	i, _ := strconv.ParseInt("123", 0, 64)
	fmt.Println(i)
	d, _ := strconv.ParseInt("0x1c8", 0, 64)
	fmt.Println(d)

	k, _ := strconv.Atoi("135")
	fmt.Println(k)
	j := strconv.Itoa(121)
	fmt.Println(j)
}
