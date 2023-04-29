package common

import (
	"fmt"
	"io/ioutil"
	"os"
	"testing"
)

func TestExample02_00(t *testing.T) {
	f := createFile("test.txt")
	defer closeFile(f)
	writeFile(f)
}

func TestExample02_01(t *testing.T) {
	data, err := ioutil.ReadFile("test.txt")
	if err != nil {
		println(err.Error())
	}
	fmt.Println(string(data))
}

func TestExample02_02(t *testing.T) {
	content := []byte("this is a test content222")
	err := ioutil.WriteFile("test.txt", content, 0644)
	if err != nil {
		println(err.Error())
	}
}

func createFile(p string) *os.File {
	fmt.Println("creating")
	f, err := os.Create(p)
	if err != nil {
		println(err.Error())
	}
	return f
}

func writeFile(f *os.File) {
	fmt.Println("writing")
	_, _ = fmt.Fprintln(f, "this is a test content")
}

func closeFile(f *os.File) {
	fmt.Println("closing")
	_ = f.Close()
}
