package day3

import (
	"log"
	"os"
	"strings"
)

type Number struct {
	start int
	end   int
	y     int
	value int
}

func parseNumbersFromLine(line string, y int) []Number {
	numbers := make([]Number, 0)
	X := len(line)

	for x := 0; x < X; x++ {
		start := x
		curr := 0
		for ; x < X && isDigit(line[x]); x++ {
			curr = curr*10 + int(line[x]-'0')
		}
		if curr != 0 {
			x--
			numbers = append(numbers, Number{start, x, y, curr})
		}
	}

	return numbers
}

func getValidNumbers(numbers []Number, x int, y int) []Number {
	validNumbers := make([]Number, 0)

	for _, number := range numbers {
		if number.y == y {
			if number.start == x+1 || number.end == x-1 {
				validNumbers = append(validNumbers, number)
			}
			continue
		}

		if number.y == y+1 || number.y == y-1 {
			if number.start <= x+1 && number.end >= x-1 {
				validNumbers = append(validNumbers, number)
			}
		}
	}

	return validNumbers
}

func getGear(x int, y int, nums []Number) int {
	validNumbers := getValidNumbers(nums, x, y)

	if len(validNumbers) == 2 {
		return validNumbers[0].value * validNumbers[1].value
	}
	return 0
}

func Part2(filepath string) int {
	input, err := os.ReadFile(filepath)
	if err != nil {
		log.Fatalf("Could not read the input file: %v", err)
	}

	sum := 0

	grid := strings.Split(string(input), "\n")
	Y := len(grid)
	X := len(grid[0])

	allNumbers := make([]Number, 0)
	for y, line := range grid {
		allNumbers = append(allNumbers, parseNumbersFromLine(line, y)...)
	}

	for y := 0; y < Y; y++ {
		for x := 0; x < X; x++ {
			if grid[y][x] == '*' {
				sum += getGear(x, y, allNumbers)
			}
		}
	}

	return sum
}
