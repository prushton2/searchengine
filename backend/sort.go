package main

// func SortURLs(self map[string]ScoredURL) []string {

// }

// func RadixSort(self []SortableScoredURL, get func(SortableScoredURL) float64) []SortableScoredURL {
// 	// get maximum value

// 	// for each digit from lowest power of 10 to largest power of 10:

// }

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

	for _, i := range self {
		value := get(i)
		output[count[value]-1] = i
		count[value] -= 1
	}
	return output
}
