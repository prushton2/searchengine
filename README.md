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
| **ENVIRONMENT**| `dev\|prod` | Environment the stack is running in |
| **FRONTEND_PORT** | `port` | What port the docker compose should expose as the frontend port |
| **BACKEND_PORT** | `port` | What port the docker compose should expose as the backend port |



# Crawler
The responsibility of the crawler is to find text and urls in a page. I use a queue with a depth limit of 3 to crawl pages, and i store the postgres db

## Issues
## Not started
* Use a real word tokenizer
* Reqwest does not scrape pages with JS rendering
* Connection Pooling
### In Progress
* Needs to respect robots.txt
    * [ ] Read crawl delay for page
    * [X] Read allowed URLs
### Resolved
* Create multiple crawlers each with a thread
* Reqwest does not resolve 300 response codes, leading to pages that can only be searched with "Permanently Moved"
    * [X] Should return the dereferenced url and use that url for indexing
    * [X] Recursively dereferences 3XX codes
* Currently, the crawler will only search a maximum depth of 4 into a website before entering another url. This doesnt work well, as any change in the url resets the depth
* It should store site specific data (title, description, etc..)

# Indexer
The indexer takes crawled data and sorts it by word. 

## Issues
### In Progress
* Half decent algorithm
### Resolved
* Batching queries
* This should use a real database
* Strip non important words
* The indexer is bad at character lengths, since characters arent well defined in unicode

# Backend
The backend gets a search request and compiles the requested sites for the frontend

## Issues
### Not Started
### In Progress
### Resolved
* Lowercase all letters in query
* It doesnt return sorted data
* Ranking should look for word occurrences in webpage


# Database Schema

## CrawledData
Table of basic site data after a crawl
| url | title | description |
| :--- | :--- | :--- |
| string | string | string |
| primary_key | |

***
## CrawledWords
Table of a word with its url and the parent element, with the amount of times it appears

| url | word | parent | count |
| :--- | :--- | :--- | :--- |
| string | string | string | int |
| primary_key | primary_key | primary_key | |

***
## URLQueue
Queue of URLs. URLs with a crawler id of 0 are up for grabs by crawlers

| url | depth | crawler_id |
| :--- | :--- | :--- |
| string | int | int |
| primary_key | | |

***
## CrawledURLs
List of crawled urls and the time they can be crawled again at

| url | crawled_again_at | |
| :--- | :--- | :--- |
| string | UNIX seconds |
| primary_key | |

***
## IndexedWords
Words with their site and the weight they have after being indexed

| url | word | weight |
| :--- | :--- | :--- |
| string | string | int |
| primary_key | primary_key | |

***
## SiteMetadata
Basic info about the site to display on the frontend

| url | title | description |
| :--- | :--- | :--- |
| string | string | string |
| primary_key | | |