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

func filter(i int, numbers []string, least bool) string {
	if len(numbers) == 1 {
		return numbers[0]
	}

	counter := 0

	for _, bits := range numbers {
		if bits[i] == '1' {
			counter += 1
		}
	}

	newNumbers := []string{}

	for _, bits := range numbers {
		if 2*counter >= len(numbers) {
			if bits[i] == '1' && !least {
				newNumbers = append(newNumbers, bits)
			} else if bits[i] == '0' && least {
				newNumbers = append(newNumbers, bits)
			}
		} else {
			if bits[i] == '0' && !least {
				newNumbers = append(newNumbers, bits)
			} else if bits[i] == '1' && least {
				newNumbers = append(newNumbers, bits)
			}
		}
	}

	return filter(i+1, newNumbers, least)
}

func part2(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	ans := 0
	numbers := []string{}
	maxLength := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		bits := scanner.Text()
		numbers = append(numbers, bits)

		maxLength = len(numbers[len(numbers)-1]) // assume all lines are the same length
	}

	o2 := filter(0, numbers, false)
	co2 := filter(0, numbers, true)

	numberO2 := 0
	numberCO2 := 0

	for i := 0; i < maxLength; i++ {
		numberO2 *= 2
		numberCO2 *= 2
		if o2[i] == '1' {
			numberO2 += 1
		}
		if co2[i] == '1' {
			numberCO2 += 1
		}
	}

	ans = numberO2 * numberCO2

	fmt.Println("Part 2: ", ans)
}

func main() {
	part1("./input.txt")
	part2("./input.txt")
}
