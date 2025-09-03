# Crawler
- Given webpage to start
- Get all text outside tags and excluding scripts
  - store important words with score
- get all links
- add links to crawler list

## Stored format
Hashmap i guess???
(what do i do fr)

each site is its own file
```json
{
    "version": 1,
    "url": "<url>",
    "words": {
        "word": <occurence>
    }
}
```

# Indexer
- finds a file in the crawlers output
- eats it
  - adds the site to the words db
  - example:
```json

[
    {
        "word": "domain",
        "sites": {
            "url": "example.com",
            "weight": 3.14
        }
    }
]
```
- im worried about loading a massive file into memory, so im going to abbreviate files with the first two letters of the words inside them
- i load one file at a time

# Backend
- Takes words in a search
- look them up in the indexer's db
- sums weights and spits out results

# frontend
- boring