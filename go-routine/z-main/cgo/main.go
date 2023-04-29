package main

/*
	#include "main.c"
	void printInt(int v);
	void printFloat(float v);
*/
import "C"

func main() {
	C.printInt(10)
	C.printFloat(10.0)
}
