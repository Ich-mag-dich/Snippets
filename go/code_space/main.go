package main

import "fmt"

func main() {
	fmt.Printf("hello %s! \n",hello("code space"))
}

func hello(arg1 string) string {
  return arg1
}
