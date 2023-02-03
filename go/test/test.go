package main

import (
	"fmt"
	"io"
	"log"
	"net/http"

	"github.com/PuerkitoBio/goquery"
)

func getHtml(url string) *http.Response {
	res, err := http.Get(url)
	if err != nil {
		log.Fatal(err)
	}
	return res
}

func getBody(res *http.Response) string {
	body, err := io.ReadAll(res.Body)
	if err != nil {
		log.Fatal(err)
	}
	return string(body)
}

func getH1(url, tag string) *goquery.Selection {
	res, err := http.Get(url)
	if err != nil {
		log.Fatal(err)
	}
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
}

func main() {
	fmt.Println("hello world!")
	var url, tag string
	var num int
	fmt.Print("1. check reponse, 2. get tag, 3. get full page\nInput number: ")
	fmt.Scanln(&num)
	fmt.Print("Input Url: ")
	fmt.Scanln(&url)
	switch num {
	case 1:
		fmt.Print(getHtml(url).Status)
	case 2:
		fmt.Print("Input tag: ")
		fmt.Scanln(&tag)
		getH1(url, tag).Each(func(i int, s *goquery.Selection) {
			fmt.Printf("%d : %s\n", i, s.Text())
		})
	case 3:
		fmt.Print(getBody(getHtml(url)))
	}
}
