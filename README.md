# Search Engine
A search engine built in Rust, Go, and React. This is purely a project for learning, I dont recommend actually using it.

# Configuration
Run `cp example.env .env` to create a template env file

Options:

| Option | Type | Description |
| :--- | :--- | :--- |
| **POSTGRES_DB_USER** | `string` | Username for the postgres db |
| **POSTGRES_DB_PASSWORD** | `string` | Password for the postgres db |
| **POSTGRES_DB_DATABASE** | `string` | db name for the postgres db |
| **POSTGRES_DB_HOST** | `string` | hostname of the postgres db |
| **MAX_CRAWL_DEPTH** | `uint8_t` | How many pages deep into a domain should the crawler go |
| **CRAWLER_THREADS** | `u32` | How many crawlers should be running at once |
| **VITE_BACKEND_URL** | `string` | URL of the backend |
| **FRONTEND_PORT** | | What port the docker compose should expose as the frontend port |
| **BACKEND_PORT** | | What port the docker compose should expose as the backend port |



# Crawler
The responsibility of the crawler is to find text and urls in a page. I use a queue with a depth limit of 3 to crawl pages, and i store the postgres db

## Issues
## Not started
* The crawler is bad at filtering words. It will concatenate words in adjacent HTML elements, and cannot distinguish hex codes
    * Ideally use a real tokenizer
* Create multiple crawlers each with a thread
    * count defined in .env
* Reqwest does not scrape pages with JS rendering
### In Progress
* Needs to respect robots.txt
    * [ ] Read crawl delay for page
    * [X] Read allowed URLs
### Resolved
* Reqwest does not resolve 300 response codes, leading to pages that can only be searched with "Permanently Moved"
    * [X] Should return the dereferenced url and use that url for indexing
    * [X] Recursively dereferences 3XX codes
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

| primary_key url | title | description |
| :--- | :--- | :--- |

***

## CrawledWords

| primary_key url | primary_key word | count |
| :--- | :--- | :--- |

***

## URLQueue

| primary_key url | depth | priority |
| :--- | :--- | :--- |

***

## CrawledURLs

| primary_key url | crawled_again_at | |
| :--- | :--- | :--- |

***

## IndexedWords

| primary_key url | primary_key word | weight |
| :--- | :--- | :--- |

***

## SiteMetadata

| primary_key url | title | description |
| :--- | :--- | :--- |