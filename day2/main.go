package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func part1(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0
	horizontal := 0
	depth := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		data := strings.Split(scanner.Text(), " ")
		switch data[0] {
		case "forward":
			number, _ := strconv.Atoi(data[1])
			horizontal += number
		case "up":
			number, _ := strconv.Atoi(data[1])
			depth -= number
		case "down":
			number, _ := strconv.Atoi(data[1])
			depth += number
		}
	}

	ans = horizontal * depth

	fmt.Println("Part 1: ", ans)
}

func part2(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0
	horizontal := 0
	depth := 0
	aim := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		data := strings.Split(scanner.Text(), " ")
		switch data[0] {
		case "forward":
			number, _ := strconv.Atoi(data[1])
			horizontal += number
			depth += number * aim
		case "up":
			number, _ := strconv.Atoi(data[1])
			aim -= number
		case "down":
			number, _ := strconv.Atoi(data[1])
			aim += number
		}
	}

	ans = horizontal * depth

	fmt.Println("Part 1: ", ans)
}

func main() {
	part1("./input.txt")
	part2("./input.txt")
}
