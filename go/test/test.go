package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
)

func getPage(link string) string {
	res, err := http.Get(link)
	if err != nil {
		log.Fatal(err)
	}
	body, err := ioutil.ReadAll(res.Body)
	res.Body.Close()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%s", body)
	return string(body)
}

func main() {
	fmt.Println("hello world!")
	var w1 string
	fmt.Scanln(&w1)
	fmt.Println(getPage(w1))

}
