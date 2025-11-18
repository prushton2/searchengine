package database

import (
	"database/sql"
	"fmt"
	"strings"

	_ "github.com/lib/pq"
	"prushton.com/search/config"
)

type SiteMetadata struct {
	Title       string `json:"title"`
	Description string `json:"description"`
}

type Database struct {
	Client   *sql.DB
	PageSize int
}

func Connect(dbinfo config.PostgresDBInfo, PageSize int) (Database, error) {
	psqlconn := fmt.Sprintf("host=%s port=5432 user=%s password=%s dbname=%s sslmode=disable", dbinfo.Host, dbinfo.Username, dbinfo.Password, dbinfo.Dbname)
	db, err := sql.Open("postgres", psqlconn)
	if err != nil {
		return Database{}, err
	}

	return Database{
		Client:   db,
		PageSize: PageSize,
	}, nil
}

func (self *Database) Get_words(word string, page int) (map[string]int64, error) {
	rows, err := self.Client.Query("SELECT * FROM indexedwords WHERE word = $1 ORDER BY weight DESC LIMIT $2", word, self.PageSize*page)

	if err != nil {
		return nil, err
	}

	var wordmap map[string]int64 = make(map[string]int64)

	for rows.Next() {

		var url string
		var word string
		var weight int

		rows.Scan(&url, &word, &weight)

		wordmap[url] = int64(weight)
	}

	return wordmap, nil
}

func (self *Database) Get_site_metadata(query_urls []string) (map[string]SiteMetadata, error) {
	if len(query_urls) == 0 {
		return make(map[string]SiteMetadata), nil
	}

	var query = fmt.Sprintf("SELECT * FROM sitemetadata WHERE url IN ('%s');", strings.Join(query_urls, "', '"))
	// fmt.Printf("q: %s\n", query)
	rows, err := self.Client.Query(query)

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
