package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/PuerkitoBio/goquery"
)

func getH1(link, tag string) {
	res, err := http.Get(link)
	if err != nil {
		log.Fatal(err)
	}
	// body, err := io.ReadAll(res.Body)
	if err != nil {
		log.Fatal(err)
	}
	doc, err := goquery.NewDocumentFromReader(res.Body)
	if err != nil {
		log.Fatal(err)
	}
	res.Body.Close()
	doc.Find(tag).Each(func(i int, s *goquery.Selection) {
		fmt.Printf("%d : %s\n", i, s.Text())
	})
	// fmt.Printf("%s", body)
}

func main() {
	fmt.Println("hello world!")
	var w1, w2 string
	fmt.Print("Input Url: ")
	fmt.Scanln(&w1)
	fmt.Print("Input tag: ")
	fmt.Scanln(&w2)
	getH1(w1, w2)

	// testdns(w1)
}
