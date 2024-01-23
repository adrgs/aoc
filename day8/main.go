package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func part1(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		numbers := strings.Split(scanner.Text(), " | ")
		for _, number := range strings.Split(numbers[1], " ") {
			if len(number) == 2 || len(number) == 3 || len(number) == 4 || len(number) == 7 {
				ans += 1
			}
		}
	}

	fmt.Println("Part 1:", ans)
}

func intersectionSize(a string, b string) int {
	aSet := make(map[rune]bool)
	bSet := make(map[rune]bool)

	for _, number := range a {
		aSet[number] = true
	}
	for _, number := range b {
		bSet[number] = true
	}

	count := 0

	for number := range aSet {
		if bSet[number] {
			count += 1
		}
	}

	return count
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
		input := scanner.Text()
		inputArray := strings.Split(input, " | ")
		known := [10]string{}
		// initial pass of the input, get the known numbers (1, 4, 7, 8)
		for _, number := range strings.Split(inputArray[0], " ") {
			if len(number) == 2 {
				known[1] = number
			} else if len(number) == 3 {
				known[7] = number
			} else if len(number) == 4 {
				known[4] = number
			} else if len(number) == 7 {
				known[8] = number
			}
		}

		// now we should be able to figure out the rest of the numbers of length 6
		for _, number := range strings.Split(inputArray[0], " ") {
			if len(number) == 6 { // if number is either 0, 6 or 9
				if intersectionSize(number, known[4]) == 3 && intersectionSize(number, known[1]) == 2 {
					known[0] = number
				} else if intersectionSize(number, known[1]) == 1 {
					known[6] = number
				} else {
					known[9] = number
				}
			}
		}

		// and finally, figure out the rest of the numbers of length 5
		for _, number := range strings.Split(inputArray[0], " ") {
			if len(number) == 5 { // if number is either 0, 6 or 9
				if intersectionSize(number, known[6]) == 5 {
					known[5] = number
				} else if intersectionSize(number, known[9]) == 5 {
					known[3] = number
				} else {
					known[2] = number
				}
			}
		}

		nr := 0
		for _, number := range strings.Split(inputArray[1], " ") {
			for i, knownNumber := range known {
				t := intersectionSize(number, knownNumber)
				if t == len(number) && t == len(knownNumber) {
					nr *= 10
					nr += i
					break
				}
			}
		}

		ans += nr
	}

	fmt.Println("Part 2:", ans)
}

func main() {
	part1("./input.txt")
	part2("./input.txt")
}
