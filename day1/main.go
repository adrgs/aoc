package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func part1(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	previous := -1
	ans := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		number, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}

		if previous != -1 {
			if number > previous {
				ans += 1
			}
		}

		previous = number

	}

	fmt.Println("Part 1: ", ans)
}

func part2(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	arr := []int{}
	ans := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		number, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}

		arr = append(arr, number)
	}

	for i := 3; i < len(arr); i++ {
		if arr[i] > arr[i-3] {
			ans += 1
		}
	}

	fmt.Println("Part 1: ", ans)
}

func main() {
	part1("./input.txt")
	part2("./input.txt")
}
