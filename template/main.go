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

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}

	ans := 0
	fmt.Println("Part 1: ", ans)
}

func part2(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}

	ans := 0
	fmt.Println("Part 1: ", ans)
}

func main() {
	name := "template"
	filename := "./" + name + "/input.txt"

	part1(filename)
	part2(filename)
}
