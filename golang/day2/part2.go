package day2

import (
	"log"
	"os"
	"strings"
)

func (d *Draw) product() int {
	return d.Red * d.Blue * d.Green
}

func Part2(filepath string) int {
	input, err := os.ReadFile(filepath)
	if err != nil {
		log.Fatalf("Could not read the input file: %v", err)
	}

	sum := 0
	lines := strings.Split(string(input), "\n")

	for _, line := range lines {
		game := newGame(line)
		minDraw := game.minDraw()
		sum += minDraw.product()
	}

	return sum
}
