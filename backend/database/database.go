package database

import (
	"database/sql"
	"fmt"
	"strings"

	_ "github.com/lib/pq"
)

type SiteMetadata struct {
	Title       string `json:"title"`
	Description string `json:"description"`
}

const (
	host     = "localhost"
	port     = 5432
	user     = "user"
	password = "password"
	dbname   = "maindb"
)

func Connect() (*sql.DB, error) {
	psqlconn := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable", host, port, user, password, dbname)
	return sql.Open("postgres", psqlconn)
}

func Get_words(db *sql.DB, word string) (map[string]float64, error) {
	rows, err := db.Query("SELECT * FROM indexedwords WHERE word = $1", word)

	if err != nil {
		return nil, err
	}

	var wordmap map[string]float64 = make(map[string]float64)

	for rows.Next() {

		var url string
		var word string
		var weight int

		rows.Scan(&url, &word, &weight)

		wordmap[url] = float64(weight)
	}

	return wordmap, nil
}

func Get_site_metadata(db *sql.DB, query_urls []string) (map[string]SiteMetadata, error) {
	var query = fmt.Sprintf("SELECT * FROM sitemetadata WHERE url IN ('%s');", strings.Join(query_urls, "', '"))
	// fmt.Printf("q: %s\n", query)
	rows, err := db.Query(query)

	if err != nil {
		return map[string]SiteMetadata{}, err
	}

	var metadata map[string]SiteMetadata = make(map[string]SiteMetadata, 0)

	for rows.Next() {
		var url string
		var title string
		var description string

		rows.Scan(&url, &title, &description)

		metadata[url] = SiteMetadata{
			Title:       title,
			Description: description,
		}

	}

	return metadata, nil
}
