# Search Engine
A search engine built in Rust, Go, and React. This is purely a project for learning, I dont recommend actually using it.

# Crawler
The responsibility of the crawler is to find text and urls in a page. I use a queue with a depth limit of 4 to crawl pages, and i store the crawled page in a file named `timestamp.json`

## Issues
* Currently, the crawler will only search a maximum depth of 4 into a website before entering another url. This doesnt work well, as any change in the url resets the depth
* The crawler is bad at filtering words. It will concatenate words in adjacent HTML elements, and cannot distinguish hex codes
* Curl does not resolve 300 response codes, leading to pages that can only be searched with "Permanently Moved"
* It should store site specific data (title, description, etc..)

# Indexer
The indexer takes crawled data and sorts it by word. A word must be 2 characters long, and is stored in `./word[0..2]/word.json`. The file contains URLs and their score with the indexer.

## Issues
* The indexer is bad at character lengths, since characters arent well defined in unicode
* It is abysmally slow. I suspect this to be the many files written in quick succession, but i have yet to benchmark it
* Fix some security issues by stripping characters

# Backend
The backend gets a search request and compiles the requested sites for the frontend

# Issues
* It doesnt return sorted data

# Frontend
The website to show the user their search query

# Issues
* It looks horrible