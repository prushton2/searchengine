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

type HTTPResponse struct {
	Urls        []string                         `json:"url"`
	Metadata    map[string]database.SiteMetadata `json:"metadata"`
	ElapsedTime int64                            `json:"elapsedtime"`
}

type ScoredURL struct {
	// summed score of all the words
	Score int64 `json:"score"`
	// amount of words in the query mentioned in the page
	OccurrencesInQuery int32 `json:"occurrencesInQuery"`
}

type SortableScoredURL struct {
	Url                string `json:"url"`
	Score              int64  `json:"score"`
	OccurrencesInQuery int64  `json:"occurrencesInQuery"`
}

var dbinfo database.DBInfo = database.DBInfo{}
var conn *sql.DB = nil

func addScoredURLs(self map[string]ScoredURL, other map[string]int64) map[string]ScoredURL {
	// iterate over the other one
	for key, otherValue := range other {
		selfValue, exists := self[key]

		if exists {
			// if exists, we add the total score and increment the number of words the url has in the query
			self[key] = ScoredURL{
				Score:              selfValue.Score + otherValue,
				OccurrencesInQuery: selfValue.OccurrencesInQuery + 1,
			}
		} else {
			// else create the entry
			self[key] = ScoredURL{
				Score:              otherValue,
				OccurrencesInQuery: 1,
			}
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
	var Scores map[string]ScoredURL = make(map[string]ScoredURL)

	for _, word := range search {
		newURLs, err := database.Get_words(conn, word)

		if err != nil {
			continue
		}

		Scores = addScoredURLs(Scores, newURLs)
	}

	fmt.Println("----- Scored URLS")
	for key, value := range Scores {
		fmt.Printf("%s: %d %d\n", key, value.Score, value.OccurrencesInQuery)
	}

	// Sort the urls by score
	SortedURLs := SortURLs(Scores)

	fmt.Println("----- Sorted URLS")
	for _, key := range SortedURLs {
		fmt.Printf("%s\n", key)
	}

	metadata, err := database.Get_site_metadata(conn, SortedURLs)

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		io.WriteString(w, "Error getting metadata")
		return
	}

	end := time.Now().UnixNano() / int64(time.Millisecond)

	var response HTTPResponse = HTTPResponse{
		Urls:        SortedURLs,
		Metadata:    metadata,
		ElapsedTime: end - start,
	}

	v, err := json.Marshal(response)
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
		fmt.Printf("Error loading dotenv, %s\nexiting", err)
		return
	}

	dbinfo.User = os.Getenv("POSTGRES_DB_USER")
	dbinfo.Host = os.Getenv("POSTGRES_DB_HOST")
	dbinfo.Password = os.Getenv("POSTGRES_DB_PASSWORD")
	dbinfo.Dbname = os.Getenv("POSTGRES_DB_DATABASE")

	// fmt.Printf("username: %s\n", os.Getenv("POSTGRES_DB_USER"))

	conn, err = database.Connect(dbinfo)

	if err != nil {
		fmt.Println("Error connecting to database, exiting")
		return
	}

	// conn = connection

	http.HandleFunc("/search", search)

	fmt.Println("Running server")
	_ = http.ListenAndServe(":3333", nil)
}
