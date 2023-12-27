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

	scanner := bufio.NewScanner(file)
	lines := []string{}

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	bingoNumbers := []int{}

	for _, number := range strings.Split(lines[0], ",") {
		iNumber, _ := strconv.Atoi(number)
		bingoNumbers = append(bingoNumbers, iNumber)
	}

	type bingo [5][5]int

	bingos := []bingo{}

	splitFn := func(c rune) bool {
		return c == ' '
	}

	for i := 2; i < len(lines); i += 6 {
		b := bingo{}
		for j := 0; j < 5; j++ {
			for k, number := range strings.FieldsFunc(lines[i+j], splitFn) {
				iNumber, _ := strconv.Atoi(number)
				b[j][k] = iNumber
			}
		}
		bingos = append(bingos, b)
	}

	markBingo := func(bingo *bingo, number int) {
		for i := 0; i < 5; i++ {
			for j := 0; j < 5; j++ {
				if bingo[i][j] == number {
					bingo[i][j] = -1
				}
			}
		}
	}

	checkBingo := func(bingo bingo) bool {
		for i := 0; i < 5; i++ {
			good := true

			for j := 0; j < 5; j++ {
				if bingo[i][j] != -1 {
					good = false
					break
				}
			}

			if good {
				return true
			}
		}

		for j := 0; j < 5; j++ {
			good := true

			for i := 0; i < 5; i++ {
				if bingo[i][j] != -1 {
					good = false
					break
				}
			}

			if good {
				return true
			}
		}

		return false
	}

	sumBingo := func(bingo bingo) int {
		sum := 0

		for i := 0; i < 5; i++ {
			for j := 0; j < 5; j++ {
				if bingo[i][j] != -1 {
					sum += bingo[i][j]
				}
			}
		}

		return sum
	}

out:
	for _, nr := range bingoNumbers {
		for i := 0; i < len(bingos); i++ {
			markBingo(&bingos[i], nr)
			if checkBingo(bingos[i]) {
				ans = sumBingo(bingos[i]) * nr
				break out
			}
		}
	}

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
