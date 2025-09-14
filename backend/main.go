package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"

	"prushton.com/search/database"
)

// type IndexedPage struct {
// 	Urls [][]any `json:"urls"`
// }

// type Metadata struct {
// 	Urls map[string]SiteMetadata `json:"urls"`
// }

type ScoredURLs struct {
	Words       map[string]float64               `json:"words"`
	Metadata    map[string]database.SiteMetadata `json:"metadata"`
	ElapsedTime int64                            `json:"elapsedtime`
}

func addScoredURLs(self map[string]float64, other map[string]float64) map[string]float64 {
	for key, value := range other {
		selfScore, exists := self[key]

		if exists {
			self[key] = value + selfScore
		} else {
			self[key] = value
		}
	}

	return self
}

func search(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
	w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

	start := time.Now().UnixNano() / int64(time.Millisecond)

	if r.Method == http.MethodOptions {
		w.WriteHeader(http.StatusOK)
		return
	}

	query := r.URL.Query()
	search := strings.Split(query.Get("s"), " ")

	conn, err := database.Connect()

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	// map of url and total score for query
	var allScores map[string]float64 = make(map[string]float64)

	for _, word := range search {
		scores, err := database.Get_words(conn, word)

		if err != nil {
			continue
		}

		allScores = addScoredURLs(allScores, scores)
	}

	var urls []string = make([]string, 0)

	for key := range allScores {
		urls = append(urls, key)
	}

	metadata, err := database.Get_site_metadata(conn, urls)

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	end := time.Now().UnixNano() / int64(time.Millisecond)

	var scoredurls ScoredURLs = ScoredURLs{
		Words:       allScores,
		Metadata:    metadata,
		ElapsedTime: end - start,
	}

	v, err := json.Marshal(scoredurls)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	io.Writer.Write(w, v)
}

func main() {
	http.HandleFunc("/search", search)

	fmt.Println("Running server")
	_ = http.ListenAndServe(":3333", nil)
}
