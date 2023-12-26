package main

import (
	"bufio"
	"fmt"
	"os"
)

func part1(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0
	counter := [64]int{}
	length := 0
	maxLength := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		length += 1
		bits := scanner.Text()
		for i, bit := range bits {
			if bit == '1' {
				counter[i] += 1
			}
		}
		maxLength = len(bits) // assume all lines are the same length
	}

	gamma := 0
	eps := 0

	for i := 0; i < maxLength; i++ {
		gamma *= 2
		eps *= 2
		if counter[i] > length/2 {
			gamma += 1
		} else {
			eps += 1
		}
	}

	ans = gamma * eps

	fmt.Println("Part 1: ", ans)
}

func part2(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {

	}

	fmt.Println("Part 1: ", ans)
}

func main() {
	part1("./input.txt")
	part2("./input.txt")
}
