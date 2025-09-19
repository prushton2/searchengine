package main

import (
	"testing"
)

var sampledata map[string]ScoredURL = map[string]ScoredURL{
	"ai domain 1": ScoredURL{
		Score:              1,
		OccurrencesInQuery: 2,
	},
	"ai domain 2": ScoredURL{
		Score:              2,
		OccurrencesInQuery: 2,
	},
	"ai domain 20": ScoredURL{
		Score:              20,
		OccurrencesInQuery: 2,
	},
	"ai domain 22": ScoredURL{
		Score:              22,
		OccurrencesInQuery: 2,
	},
	"ai 1": ScoredURL{
		Score:              1,
		OccurrencesInQuery: 1,
	},
	"ai 2": ScoredURL{
		Score:              2,
		OccurrencesInQuery: 1,
	},
	"domain 1": ScoredURL{
		Score:              1,
		OccurrencesInQuery: 1,
	},
	"domain 2": ScoredURL{
		Score:              2,
		OccurrencesInQuery: 1,
	},
}

var sampleSortableScoredURLs []SortableScoredURL = []SortableScoredURL{
	SortableScoredURL{
		Url:                "ai domain 1",
		Score:              1,
		OccurrencesInQuery: 2,
	},
	SortableScoredURL{
		Url:                "ai domain 2",
		Score:              2,
		OccurrencesInQuery: 2,
	},
	SortableScoredURL{
		Url:                "ai domain 20",
		Score:              20,
		OccurrencesInQuery: 2,
	},
	SortableScoredURL{
		Url:                "ai domain 22",
		Score:              22,
		OccurrencesInQuery: 2,
	},
	SortableScoredURL{
		Url:                "ai 1",
		Score:              1,
		OccurrencesInQuery: 1,
	},
	SortableScoredURL{
		Url:                "ai 2",
		Score:              2,
		OccurrencesInQuery: 1,
	},
	SortableScoredURL{
		Url:                "domain 1",
		Score:              1,
		OccurrencesInQuery: 1,
	},
	SortableScoredURL{
		Url:                "domain 2",
		Score:              2,
		OccurrencesInQuery: 1,
	},
}

func TestCountingSort(t *testing.T) {
	sorted := CountingSort(sampleSortableScoredURLs, func(ssu SortableScoredURL) int64 { return ssu.Score % 10 })
	var last int64 = 0

	for _, i := range sorted {
		if i.Score%10 < last {
			t.Errorf("Counting Sort failed")
		}
		last = i.Score % 10
	}
}
