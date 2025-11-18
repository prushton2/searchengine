package main

import (
	"encoding/json"
	"fmt"
	"io"
	"math"
	"net/http"
	"regexp"
	"strconv"
	"strings"
	"time"

	"prushton.com/search/config"
	"prushton.com/search/database"
)

type HTTPResponse struct {
	Urls         []string                         `json:"url"`
	Metadata     map[string]database.SiteMetadata `json:"metadata"`
	ElapsedTime  int64                            `json:"elapsedtime"`
	TotalResults int64                            `json:"totalResults"`
	PageNo       int64                            `json:"pageno"`
	PageSize     int64                            `json:"pagesize"`
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

var conf config.Config = config.Config{}
var db database.Database = database.Database{Client: nil}

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

	var nonAlphanumericRegex = regexp.MustCompile(`[^a-zA-Z0-9 ]+`)

	query := r.URL.Query()
	rawSearch := query.Get("s")
	rawSearch = nonAlphanumericRegex.ReplaceAllString(rawSearch, " ")
	rawSearch = strings.ToLower(rawSearch)
	search := strings.Split(rawSearch, " ")

	// fmt.Printf("Raw search: %s\n", rawSearch)
	// fmt.Printf("Search: %s\n", search)

	pageNo, err := strconv.ParseInt(query.Get("p"), 10, 64)
	if err != nil {
		pageNo = 1
	}

	// fmt.Printf("Page %d\n", pageNo)

	if db.Client == nil {
		w.WriteHeader(http.StatusInternalServerError)
		io.WriteString(w, "No database connection found")
		return
	}

	// map of url and total score for query
	var Scores map[string]ScoredURL = make(map[string]ScoredURL)

	for _, word := range search {
		newURLs, err := db.Get_words(word, int(pageNo))

		if err != nil {
			fmt.Printf("Error %s\n", err)
			continue
		}

		Scores = addScoredURLs(Scores, newURLs)
	}

	// Sort the urls by score
	SortedURLs := SortURLs(Scores)

	// 50 is the page size, we are clamping the upper cap inside the size of the array and making sure lowercap is 50 less than uppercap or 0
	upperCap := int64(math.Min(float64(pageNo*int64(db.PageSize)), float64(len(SortedURLs))))
	lowerCap := int64(math.Max(float64((pageNo-1)*int64(db.PageSize)), 0.0))

	TrimmedURLs := SortedURLs[lowerCap:upperCap]

	metadata, err := db.Get_site_metadata(TrimmedURLs)

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
		PageNo:       pageNo,
		PageSize:     int64(db.PageSize),
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

	conf, err = config.ReadFromFile("../config.yaml")
	if err != nil {
		fmt.Printf("Error loading config with error '%s'", err)
		return
	}

	fmt.Printf(" ----- Database Authentication ----- \n")
	fmt.Printf("username: %s\n", conf.Database.Username)
	fmt.Printf("password: %s\n", conf.Database.Password)
	fmt.Printf("host:     %s\n", conf.Database.Host)
	fmt.Printf("dbname:   %s\n", conf.Database.Dbname)

	db, err = database.Connect(conf.Database, 50)

	if err != nil {
		fmt.Println("Error connecting to database, exiting")
		return
	}

	http.HandleFunc("/search", search)

	fmt.Println("Running server")
	_ = http.ListenAndServe(":3333", nil)
}
