package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/joho/godotenv"
	"prushton.com/search/database"
)

type ScoredURLs struct {
	Words       map[string]float64               `json:"words"`
	Metadata    map[string]database.SiteMetadata `json:"metadata"`
	ElapsedTime int64                            `json:"elapsedtime"`
}

var dbinfo database.DBInfo = database.DBInfo{}
var conn *sql.DB = nil

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

	if conn == nil {
		w.WriteHeader(http.StatusInternalServerError)
		io.WriteString(w, "No database connection found")
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
		io.WriteString(w, "Error getting metadata")
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
		io.WriteString(w, "Error serializing object")
		return
	}

	io.Writer.Write(w, v)
}

func main() {
	err := godotenv.Load()

	if err != nil {
		fmt.Println("Error loading dotenv, exiting")
		return
	}

	dbinfo.User = os.Getenv("POSTGRES_DB_USER")
	dbinfo.Host = os.Getenv("POSTGRES_DB_HOST")
	dbinfo.Password = os.Getenv("POSTGRES_DB_PASSWORD")
	dbinfo.Dbname = os.Getenv("POSTGRES_DB_DATABASE")

	conn, err = database.Connect(dbinfo)

	if err != nil {
		fmt.Println("Error connecting to database, exiting")
		return
	}

	http.HandleFunc("/search", search)

	fmt.Println("Running server")
	_ = http.ListenAndServe(":3333", nil)
}
