package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func (s *Solver) Day1Part1(filepath string) int {
	input, err := os.ReadFile(filepath)
	if err != nil {
		log.Fatalf("Could not read the input file: %v", err)
	}

	lines := strings.Split(string(input), "\n")
	sum := 0

	for _, line := range lines {
		digits := make([]int, 0)
		for _, char := range line {
			if char >= '0' && char <= '9' {
				digits = append(digits, int(char-'0'))
			}
		}

		sum += (10*digits[0] + digits[len(digits)-1])
	}

	return sum
}

func (s *Solver) Day1Part2(filepath string) int {
	input, err := os.ReadFile(filepath)
	if err != nil {
		log.Fatalf("Could not read the input file: %v", err)
	}

	lines := strings.Split(string(input), "\n")
	sum := 0

	mapping := map[string]string{
		"0": "zero",
		"1": "one",
		"2": "two",
		"3": "three",
		"4": "four",
		"5": "five",
		"6": "six",
		"7": "seven",
		"8": "eight",
		"9": "nine",
	}

	for _, line := range lines {
		for k, v := range mapping {
			line = strings.Replace(line, v, fmt.Sprintf("%v%v%v", v, k, v), 100)
		}

		digits := make([]int, 0)
		for _, char := range line {
			if char >= '0' && char <= '9' {
				digits = append(digits, int(char-'0'))
			}
		}

		sum += (10*digits[0] + digits[len(digits)-1])
	}

	return sum
}
