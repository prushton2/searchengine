package main

import (
	"fmt"
	"net/http"
	"strings"

	"prushton.com/search/database"
)

// type IndexedPage struct {
// 	Urls [][]any `json:"urls"`
// }

// type Metadata struct {
// 	Urls map[string]SiteMetadata `json:"urls"`
// }

type ScoredURLs struct {
	Words    map[string]float64               `json:"words"`
	Metadata map[string]database.SiteMetadata `json:"metadata"`
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

	var allScores map[string]float64 = make(map[string]float64)

	for _, word := range search {
		scores, err := database.Get_words(conn, word)

		if err != nil {
			continue
		}

		allScores = addScoredURLs(allScores, scores)
	}

	var metadata = database.Get_site_metadata()

	var scoredurls ScoredURLs = ScoredURLs{
		Words: allScores,
	}

}

func main() {
	http.HandleFunc("/search", search)

	fmt.Println("Running server")
	_ = http.ListenAndServe(":3333", nil)
}
