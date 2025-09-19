package main

import "fmt"

// func SortURLs(self map[string]ScoredURL) []string {

// }

func RadixSort(self []SortableScoredURL, get func(SortableScoredURL) int64) []SortableScoredURL {

	// get maximum value
	var maximum int64 = 0
	var exponent int64 = 1
	for _, i := range self {
		if get(i) > maximum {
			maximum = get(i)
		}
	}
	fmt.Printf("maximum: %d\n", maximum)

	// for each digit from lowest power of 10 to largest power of 10, update sorted
	var sorted []SortableScoredURL = self

	// run the sort the required number of times
	for exponent <= maximum {
		sorted = CountingSort(sorted, func(ssu SortableScoredURL) int64 { return (get(ssu) / exponent) % 10 })
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
