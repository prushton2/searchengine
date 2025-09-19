package main

// import "fmt"

func SortURLs(self map[string]ScoredURL) []string {
	var sortableURLs []SortableScoredURL = make([]SortableScoredURL, 0)

	for key, value := range self {
		sortableURLs = append(sortableURLs, SortableScoredURL{
			Url:                key,
			Score:              int64(value.Score),
			OccurrencesInQuery: int64(value.OccurrencesInQuery),
		})
	}

	if len(sortableURLs) == 0 {
		return []string{}
	}

	if len(sortableURLs) == 1 {
		return []string{sortableURLs[0].Url}
	}

	// first we sort by score
	sortableURLs = RadixSort(sortableURLs, func(ssu SortableScoredURL) int64 { return ssu.Score })
	// then we sort by occurrences in query
	sortableURLs = RadixSort(sortableURLs, func(ssu SortableScoredURL) int64 { return ssu.OccurrencesInQuery })

	var sortedURLArray []string = make([]string, len(sortableURLs))

	for i, url := range sortableURLs {
		sortedURLArray[i] = url.Url
		// fmt.Printf("%d, %d: %s\n", url.OccurrencesInQuery, url.Score, url.Url)
	}

	for i, j := 0, len(sortedURLArray)-1; i < j; i, j = i+1, j-1 {
		sortedURLArray[i], sortedURLArray[j] = sortedURLArray[j], sortedURLArray[i]
	}

	return sortedURLArray
}

func RadixSort(self []SortableScoredURL, get func(SortableScoredURL) int64) []SortableScoredURL {

	// get maximum value
	var maximum int64 = 0
	var exponent int64 = 1
	for _, i := range self {
		if get(i) > maximum {
			maximum = get(i)
		}
	}

	// for each digit from lowest power of 10 to largest power of 10, update sorted
	var sorted []SortableScoredURL = self

	// run the sort the required number of times
	for exponent <= maximum {
		sorted = CountingSort(sorted, func(ssu SortableScoredURL) int64 { return (get(ssu) / exponent) % 10 })
		// for _, url := range sorted {
		// 	fmt.Printf("%d, %d: %s\n", url.OccurrencesInQuery, url.Score, url.Url)
		// }
		exponent *= 10
	}

	return sorted
}

func CountingSort(self []SortableScoredURL, get func(SortableScoredURL) int64) []SortableScoredURL {
	var output []SortableScoredURL = make([]SortableScoredURL, len(self))
	var count []int = make([]int, 10)

	for i := range count {
		count[i] = 0
	}

	for _, i := range self {
		var index = get(i)
		count[index] = count[index] + 1
	}

	for i := range count {
		if i == 0 {
			continue
		}
		count[i] = count[i-1] + count[i]
	}

	for i := len(self) - 1; i >= 0; i-- {
		value := get(self[i])
		output[count[value]-1] = self[i]
		count[value] -= 1
	}
	return output
}
