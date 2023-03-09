package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"
	"strconv"
	"strings"

	"github.com/PuerkitoBio/goquery"
)

var (
	fullFilePath string
	fileName     string
	title        string
)

func main() {
	var startNumber, endNumber int
	var name string
	fmt.Print("input name: ")
	fmt.Scanln(&name)
	fmt.Print("input start number: ")
	if _, err := fmt.Scanln(&startNumber); err != nil {
		fmt.Printf("\n!!! %s !!!\n", err)
		fmt.Println("!!! please input int value !!!")
		os.Exit(0)
	}
	fmt.Print("input end number: ")
	if _, err := fmt.Scanln(&endNumber); err != nil {
		fmt.Printf("\n!!! %s !!!\n", err)
		fmt.Println("!!! please input int value !!!")
		os.Exit(0)
	}
	if check := isThereFolder(name); check {
		fmt.Printf("is folder exist: %t\n", check)
	} else {
		fmt.Printf("is folder exist: %t\n", check)
		fmt.Printf("making dir...\n %s\n", name)
		os.MkdirAll(strings.Join([]string{"./", "save/", name}, ""), os.ModePerm)
	}
	titleId := getWebtoonTitleId(name)
	fmt.Println(titleId)
	for i := startNumber; i <= endNumber; i++ {
		imageUrls := getImageUrls(titleId, i)
		if check := isThereFolder2(name, i); !check {
			os.MkdirAll(strings.Join([]string{"./", "save/", name, "/", strconv.Itoa(i), " - ", title}, ""), os.ModePerm)
		}
		for num, str := range imageUrls {
			fullFilePath = strings.Join([]string{"./", "save/", name, "/", strconv.Itoa(i), " - ", title, "/"}, "")
			fileName = strings.Join([]string{fullFilePath, strconv.Itoa(num), ".jpg"}, "")

			downloadFile(str, fileName)
		}
	}
}

func downloadFile(url string, name string) error {
	fmt.Println(url, name, "downloading...")
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Fatal(err)
	}
	req.Header.Add("Accept", "*/*")
	req.Header.Add("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:65.0) Gecko/20100101 Firefox/65.0")

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer res.Body.Close()

	if res.StatusCode != 200 {
		fmt.Println("error")
		fmt.Println(fullFilePath)
		os.Exit(0)
	}
	file, err := os.Create(name)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	_, err = io.Copy(file, res.Body)
	if err != nil {
		log.Fatal(err)
	}
	return nil
}

func isThereFolder(folderName string) bool {
	path := strings.Join([]string{"./", folderName}, "")
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return false
	} else {
		return true
	}
}

func isThereFolder2(folderName string, childFolderName int) bool {
	path := strings.Join([]string{"./", folderName, "/", strconv.Itoa(childFolderName)}, "")
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return false
	} else {
		return true
	}
}

func getWebtoonTitleId(name string) string {
	var titleId string
	url := strings.Join([]string{"https://search.naver.com/search.naver?query=", url.QueryEscape(name)}, "")
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Fatal(err)
	}
	req.Header.Add("Accept", "*/*")
	req.Header.Add("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:65.0) Gecko/20100101 Firefox/65.0")

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer res.Body.Close()
	htmls, _ := goquery.NewDocumentFromReader(res.Body)
	aTag := htmls.Find("div.title_area.type_keep._title_area").Find("h2").Find("span.area_text_title").Find("strong").Find("a")
	aTag.Each(func(i int, s *goquery.Selection) {
		href, sBool := s.Attr("href")

		isContain := strings.Contains(href, "/webtoon/list.nhn?titleId=")
		isNotContain := strings.Contains(href, "&no")
		isSameName := strings.Contains(s.Text(), name)
		if sBool && isContain && isSameName && !isNotContain {
			titleId = strings.ReplaceAll(href, "https://comic.naver.com/webtoon/list.nhn?titleId=", "")
		}
	})
	return titleId
}

func getImageUrls(titleId string, num int) []string {
	var imageUrls []string
	url := strings.Join([]string{"https://comic.naver.com/webtoon/detail?titleId=", titleId, "&no=", strconv.Itoa(num)}, "")
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Fatal(err)
	}
	req.Header.Add("Accept", "*/*")
	req.Header.Add("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:65.0) Gecko/20100101 Firefox/65.0")
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		log.Fatal(err)
	} // #subTitle_toolbar
	defer res.Body.Close()
	htmls, _ := goquery.NewDocumentFromReader(res.Body)
	title = htmls.Find("#subTitle_toolbar").Text()
	title = nameReplace(title)
	imageTags := htmls.Find("div.wt_viewer").Find("img")
	imageTags.Each(func(i int, s *goquery.Selection) {
		src, _ := s.Attr("src")
		imageUrls = append(imageUrls, src)
	})
	return imageUrls
}

func nameReplace(name string) string {
	var forbiddenWords [10]string
	forbiddenWords[0] = `\`
	forbiddenWords[1] = "/"
	forbiddenWords[2] = ":"
	forbiddenWords[3] = "*"
	forbiddenWords[4] = "?"
	forbiddenWords[5] = `"`
	forbiddenWords[6] = "<"
	forbiddenWords[7] = ">"
	forbiddenWords[8] = "|"
	forbiddenWords[9] = " "
	for i := 0; i <= 9; i++ {
		if isTitleError := strings.Contains(name, forbiddenWords[i]); isTitleError {
			name = strings.ReplaceAll(name, forbiddenWords[i], "")
		}
	}
	return name
}
