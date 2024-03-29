package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type segment struct {
	x1, y1, x2, y2 int
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func sign(a int) int {
	if a < 0 {
		return -1
	} else if a > 0 {
		return 1
	}
	return 0
}

func part1(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0

	var matrix [1000][1000]int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		s := strings.Split(scanner.Text(), " -> ")
		segment := segment{}
		fmt.Sscanf(s[0], "%d,%d", &segment.x1, &segment.y1)
		fmt.Sscanf(s[1], "%d,%d", &segment.x2, &segment.y2)

		if segment.y1 == segment.y2 {
			for i := min(segment.x1, segment.x2); i <= max(segment.x1, segment.x2); i++ {
				matrix[segment.y1][i]++
			}
		} else if segment.x1 == segment.x2 {
			for i := min(segment.y1, segment.y2); i <= max(segment.y1, segment.y2); i++ {
				matrix[i][segment.x1]++
			}
		}
	}

	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			if matrix[i][j] >= 2 {
				ans++
			}
		}
	}

	fmt.Println("Part 1:", ans)
}

func part2(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0

	var matrix [1000][1000]int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		s := strings.Split(scanner.Text(), " -> ")
		segment := segment{}
		fmt.Sscanf(s[0], "%d,%d", &segment.x1, &segment.y1)
		fmt.Sscanf(s[1], "%d,%d", &segment.x2, &segment.y2)

		if segment.y1 == segment.y2 {
			for j := min(segment.x1, segment.x2); j <= max(segment.x1, segment.x2); j++ {
				matrix[segment.y1][j]++
			}
		} else if segment.x1 == segment.x2 {
			for i := min(segment.y1, segment.y2); i <= max(segment.y1, segment.y2); i++ {
				matrix[i][segment.x1]++
			}
		} else {
			j, i := segment.x1, segment.y1
			for {
				matrix[i][j]++

				j += sign(segment.x2 - segment.x1)
				i += sign(segment.y2 - segment.y1)
				if j == segment.x2 && i == segment.y2 {
					matrix[i][j]++
					break
				}
			}
		}
	}

	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			if matrix[i][j] >= 2 {
				ans++
			}
		}
	}

	fmt.Println("Part 2:", ans)
}

func main() {
	part1("./input.txt")
	part2("./input.txt")
}
