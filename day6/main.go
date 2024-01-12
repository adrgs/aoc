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
	lanternfish := make([]int, 0)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text := scanner.Text()
		for _, val := range strings.Split(text, ",") {
			number, err := strconv.Atoi(val)
			if err != nil {
				continue
			}
			lanternfish = append(lanternfish, number)
		}
	}

	for i := 0; i < 80; i++ {
		l := len(lanternfish)
		for j := 0; j < l; j++ {
			lanternfish[j]--
			if lanternfish[j] < 0 {
				lanternfish[j] = 6
				lanternfish = append(lanternfish, 8)
			}
		}
	}

	ans = len(lanternfish)

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
