// Package main provides ...
package main

import (
	"bufio"
	"fmt"
	"os"
)

func getIdx(r rune) int {
	if int(r) > 90 {
		return int(r) - 97
	} else {
		return int(r) - 39
	}
}

func main() {
	file, err := os.Open("./03.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	fileScanner := bufio.NewScanner(file)
	var elves [][52]bool
	for fileScanner.Scan() {
		var elf [52]bool
		for _, c := range fileScanner.Text() {
			idx := getIdx(c)
			if idx >= 52 {
				fmt.Println(string(c), c, idx)
			}
			elf[idx] = true
		}
		elves = append(elves, elf)
	}
	total := 0
	for i := 0; i < len(elves); i += 3 {
		for j := 0; j < 52; j++ {
			if elves[i][j] && elves[i+1][j] && elves[i+2][j] {
				total += j + 1
				break
			}
		}
	}
	fmt.Println(total)
}

// func main() {
// 	file, err := os.Open("./03.txt")
// 	if err != nil {
// 		panic(err)
// 	}
// 	defer file.Close()

// 	fileScanner := bufio.NewScanner(file)
// 	var dupes []rune
// 	for fileScanner.Scan() {
// 		txt := fileScanner.Text()
// 		mid := len(txt) / 2
// 		left := txt[:mid]
// 		right := txt[mid:]
// 		for _, c := range right {
// 			if strings.Contains(left, string(c)) {
// 				dupes = append(dupes, c)
// 				break
// 			}
// 		}
// 	}
// 	total := 0
// 	for _, r := range dupes {
// 		var score int
// 		if r > 97 {
// 			score = int(r) - 96
// 		} else {
// 			score = int(r) - 38
// 		}
// 		total += score
// 	}
// 	fmt.Println(total)
// }
