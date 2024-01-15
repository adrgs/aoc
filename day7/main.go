package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func min(x, y int) int {
	if x < y {
		return x
	}
	return y
}

func part1(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0
	crabs := make([]int, 0)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		for _, c := range strings.Split(scanner.Text(), ",") {
			number, err := strconv.Atoi(c)
			if err != nil {
				continue
			}
			crabs = append(crabs, number)
		}
	}

	// sort the crabs
	sort.Ints(crabs)

	// given len(crabs) == 1000, O(N^2) is fine

	ans = 1e9 + 7

	for i := 0; i < len(crabs); i++ {
		sum := 0
		for j := 0; j < len(crabs); j++ {
			sum += abs(crabs[i] - crabs[j])
		}
		ans = min(ans, sum)
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
	crabs := make([]int, 0)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		for _, c := range strings.Split(scanner.Text(), ",") {
			number, err := strconv.Atoi(c)
			if err != nil {
				continue
			}
			crabs = append(crabs, number)
		}
	}

	// sort the crabs
	sort.Ints(crabs)

	// given len(crabs) == 1000, O(N^2) is fine

	ans = 1e9 + 7

	for i := 0; i < len(crabs); i++ {
		sum := 0
		for j := 0; j < len(crabs); j++ {
			n := abs(crabs[i] - crabs[j])
			sum += (n * (n + 1)) / 2
		}
		ans = min(ans, sum)
	}

	fmt.Println("Part 2:", ans)
}

func main() {
	part1("./input.txt")
	part2("./input.txt")
}
