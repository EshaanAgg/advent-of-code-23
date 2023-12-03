package day3

import (
	"log"
	"os"
	"strings"
)

func isDigit(b byte) bool {
	return b >= '0' && b <= '9'
}

func isSymbol(b byte) bool {
	return !isDigit(b) && b != '.'
}

func check(start int, end int, y int, grid []string) bool {
	Y := len(grid)
	X := len(grid[0])

	allowedY := make([]int, 0)
	if y-1 >= 0 {
		allowedY = append(allowedY, y-1)
	}
	if y+1 < Y {
		allowedY = append(allowedY, y+1)
	}

	for x := max(0, start-1); x <= min(X-1, end+1); x++ {
		for _, y := range allowedY {
			if isSymbol(grid[y][x]) {
				return true
			}
		}
	}

	if start > 0 && isSymbol(grid[y][start-1]) {
		return true
	}

	if end+1 < X && isSymbol(grid[y][end+1]) {
		return true
	}

	return false
}

func Part1(filepath string) int {
	input, err := os.ReadFile(filepath)
	if err != nil {
		log.Fatalf("Could not read the input file: %v", err)
	}

	sum := 0

	grid := strings.Split(string(input), "\n")
	Y := len(grid)
	X := len(grid[0])

	for y := 0; y < Y; y++ {
		for x := 0; x < X; x++ {
			start := x
			curr := 0
			for ; x < X && isDigit(grid[y][x]); x++ {
				curr = curr*10 + int(grid[y][x]-'0')
			}
			if curr != 0 {
				x--
			}

			if check(start, x, y, grid) {
				sum += curr
			}
		}
	}

	return sum
}
