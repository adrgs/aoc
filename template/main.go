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

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
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
