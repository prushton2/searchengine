package database

import (
	"database/sql"
	"fmt"

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

func Get_site_metadata(db *sql.DB, query_urls []string) (SiteMetadata, error) {
	row := db.QueryRow("SELECT * FROM sitemetadata WHERE url = $1", query_url)

	var url string
	var title string
	var description string

	row.Scan(&url, &title, &description)

	var metadata SiteMetadata = SiteMetadata{
		Title:       title,
		Description: description,
	}

	return metadata, nil
}
