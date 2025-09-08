package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
)

type IndexedPage struct {
	Urls [][]any `json:"urls"`
}

type ScoredURLs struct {
	Urls map[string]float64 `json:"urls"`
}

func addScoredURLs(self ScoredURLs, other ScoredURLs) ScoredURLs {
	for key, value := range other.Urls {
		selfScore, exists := self.Urls[key]

		if exists {
			self.Urls[key] = value + selfScore
		} else {
			self.Urls[key] = value
		}
	}

	return self
}

func search(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query()
	search := strings.Split(query.Get("s"), " ")
	fmt.Fprintf(w, "Search terms: %v\n", search)

	var scoredURLs ScoredURLs = ScoredURLs{
		Urls: make(map[string]float64),
	}

	for _, word := range search {
		scoredURLs = addScoredURLs(get_word_score(word), scoredURLs)
	}

	bytes, _ := json.Marshal(scoredURLs)

	io.Writer.Write(w, bytes)
}

func get_word_score(word string) ScoredURLs {
	var url = fmt.Sprintf("../indexer_data/indexed_sites/%s/%s.json", word[0:2], word)
	contents, err := os.ReadFile(url)

	if err != nil {
		fmt.Printf("	Err: %s\n", err)
	}

	var indexedPage IndexedPage
	err = json.Unmarshal(contents, &indexedPage)

	if err != nil {
		fmt.Printf("Err: %s\n", err)
	}

	var scoredURLs ScoredURLs = ScoredURLs{
		Urls: make(map[string]float64),
	}

	for _, page := range indexedPage.Urls {
		scoredURLs.Urls[page[0].(string)] = page[1].(float64)
	}

	return scoredURLs
}

func main() {
	http.HandleFunc("/search", search)

	fmt.Println("Running server")
	_ = http.ListenAndServe(":3333", nil)
}
