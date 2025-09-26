package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"math"
	"net/http"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"

	"github.com/joho/godotenv"
	"prushton.com/search/database"
)

type HTTPResponse struct {
	Urls         []string                         `json:"url"`
	Metadata     map[string]database.SiteMetadata `json:"metadata"`
	ElapsedTime  int64                            `json:"elapsedtime"`
	TotalResults int64                            `json:"totalResults"`
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
	w.Header().Set("Vary", "Origin")
	w.Header().Set("Vary", "Access-Control-Request-Method")
	w.Header().Set("Vary", "Access-Control-Request-Headers")
	w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Origin, Accept, token")
	w.Header().Set("Access-Control-Allow-Methods", "GET, POST,OPTIONS")

	if r.Method == http.MethodOptions {
		w.WriteHeader(http.StatusOK)
		return
	}

	start := time.Now().UnixNano() / int64(time.Millisecond)

	query := r.URL.Query()
	rawSearch := query.Get("s")
	var nonAlphanumericRegex = regexp.MustCompile(`[^a-zA-Z0-9 ]+`)
	rawSearch = nonAlphanumericRegex.ReplaceAllString(rawSearch, " ")
	search := strings.Split(rawSearch, " ")

	pageNo, err := strconv.ParseInt(query.Get("p"), 10, 64)
	if err != nil {
		pageNo = 1
	}

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

	// Sort the urls by score
	SortedURLs := SortURLs(Scores)

	// 50 is the page size, we are clamping the upper cap inside the size of the array and making sure lowercap is 50 less than uppercap or 0
	upperCap := int64(math.Min(float64(pageNo*50), float64(len(SortedURLs))))
	lowerCap := int64(math.Max(float64((pageNo-1)*50), 0.0))

	TrimmedURLs := SortedURLs[lowerCap:upperCap]

	metadata, err := database.Get_site_metadata(conn, TrimmedURLs)

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		io.WriteString(w, "Error getting metadata")
		return
	}

	end := time.Now().UnixNano() / int64(time.Millisecond)

	var response HTTPResponse = HTTPResponse{
		Urls:         TrimmedURLs,
		Metadata:     metadata,
		ElapsedTime:  end - start,
		TotalResults: int64(len(SortedURLs)),
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
	var err error

	err = godotenv.Load()
	if err != nil {
		fmt.Printf("Error running godotenv.Load() with error '%s'; Assuming env is provided via docker\n", err)
	}

	dbinfo.User = os.Getenv("POSTGRES_DB_USER")
	dbinfo.Host = os.Getenv("POSTGRES_DB_HOST")
	dbinfo.Password = os.Getenv("POSTGRES_DB_PASSWORD")
	dbinfo.Dbname = os.Getenv("POSTGRES_DB_DATABASE")

	fmt.Printf("username: %s\n", os.Getenv("POSTGRES_DB_USER"))
	fmt.Printf("password: %s\n", os.Getenv("POSTGRES_DB_PASSWORD"))
	fmt.Printf("host:     %s\n", os.Getenv("POSTGRES_DB_HOST"))
	fmt.Printf("database: %s\n", os.Getenv("POSTGRES_DB_DATABASE"))

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
