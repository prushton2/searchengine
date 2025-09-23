# Search Engine
A search engine built in Rust, Go, and React. This is purely a project for learning, I dont recommend actually using it.

# Sample env
```.env
POSTGRES_DB_USER=user
POSTGRES_DB_PASSWORD=password
POSTGRES_DB_DATABASE=maindb
POSTGRES_DB_HOST=db
MAX_CRAWL_DEPTH=5
VITE_BACKEND_URL=http://localhost:3333
FRONTEND_PORT=8080
BACKEND_PORT=3333
```

# Crawler
The responsibility of the crawler is to find text and urls in a page. I use a queue with a depth limit of 3 to crawl pages, and i store the postgres db

## Issues
## Not started
* The crawler is bad at filtering words. It will concatenate words in adjacent HTML elements, and cannot distinguish hex codes
    * Ideally use a real tokenizer
* Create multiple crawlers each with a thread
    * count defined in .env
* Reqwest does not scrape pages with JS rendering
* Reqwest 3XX Codes
    * Should return the dereferenced url and use that url for indexing
### In Progress
* Needs to respect robots.txt
    * Doesnt read the crawl delay (if any)
### Resolved
* Reqwest does not resolve 300 response codes, leading to pages that can only be searched with "Permanently Moved"
    * Recursively dereferences 3XX codes
* Currently, the crawler will only search a maximum depth of 4 into a website before entering another url. This doesnt work well, as any change in the url resets the depth
* It should store site specific data (title, description, etc..)

# Indexer
The indexer takes crawled data and sorts it by word. 

## Issues
### In Progress
* It is pretty slow. I suspect this to be the many non batched postgres queries, but i have yet to benchmark it
### Resolved
* This should use a real database
* Strip non important words
* The indexer is bad at character lengths, since characters arent well defined in unicode

# Backend
The backend gets a search request and compiles the requested sites for the frontend

## Issues
### In Progress
### Resolved
* It doesnt return sorted data
* Ranking should look for word occurrences in webpage

# Frontend
The website to show the user their search query

## Issues
* NON ISSUE: It looks horrible


# Database Schema

## CrawledData
```
{
    primary_key url: string 
    title: string
    description: string
}
```
## CrawledWords
```
{
    primary_key url: string
    primary_key word: string
    count: int
}
```

## URLQueue
```
{
    primary_key url: string
    depth: int // only search 5 or so pages deep
    priority: int // 0 means its part of the currently scraped url, 1 means its a different tld and should be scraped later
}
```

## CrawledURLs
```
{
    primary_key url: string,
    crawled_again_at: int
}
```


## IndexedWords
```
{
    primary_key url: string
    primary_key word: string
    weight: int
}
```

## SiteMetadata
```
{
    primary_key url: string
    title: string
    description: string
}
```