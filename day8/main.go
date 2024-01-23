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

func part2(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}

	fmt.Println("Part 2:", ans)
}

func main() {
	part1("./input.txt")
	part2("./input.txt")
}
