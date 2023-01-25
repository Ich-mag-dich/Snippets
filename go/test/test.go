package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/PuerkitoBio/goquery"
)

func getH1(url, tag string) *goquery.Selection {
	res, err := http.Get(url)
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
	numAndTexts := doc.Find(tag)
	return numAndTexts
	// fmt.Printf("%s", body)
}

func main() {
	fmt.Println("hello world!")
	var url, tag string
	fmt.Print("Input Url: ")
	fmt.Scanln(&url)
	fmt.Print("Input tag: ")
	fmt.Scanln(&tag)
	getH1(url, tag).Each(func(i int, s *goquery.Selection) {
		fmt.Printf("%d : %s\n", i, s.Text())

	})

	// testdns(w1)
}
