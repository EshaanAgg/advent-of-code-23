package day2

import (
	"log"
	"os"
	"strconv"
	"strings"
)

type Draw struct {
	Red   int
	Blue  int
	Green int
}

func newDraw(s string) Draw {
	draw := Draw{}

	parts := strings.Split(s, ",")
	for _, part := range parts {
		comp := strings.Split(strings.Trim(part, " "), " ")

		i, err := strconv.Atoi(comp[0])
		if err != nil {
			log.Fatalf("Could not convert string to int while parsing count of balls in `newDraw`: %v", err)
		}

		if strings.Trim(comp[1], " ") == "red" {
			draw.Red = i
			continue
		}

		if strings.Trim(comp[1], " ") == "blue" {
			draw.Blue = i
			continue
		}

		if strings.Trim(comp[1], " ") == "green" {
			draw.Green = i
		}
	}

	return draw
}

func (d *Draw) accomodate(d2 *Draw) {
	d.Red = max(d.Red, d2.Red)
	d.Blue = max(d.Blue, d2.Blue)
	d.Green = max(d.Green, d2.Green)
}

func (d *Draw) canBeAccomodatedWith(d2 *Draw) bool {
	return d.Red <= d2.Red && d.Blue <= d2.Blue && d.Green <= d2.Green
}

type Game struct {
	ID    int
	Draws []Draw
}

func (g *Game) minDraw() Draw {
	d := Draw{}
	for _, draw := range g.Draws {
		d.accomodate(&draw)
	}
	return d
}

func newGame(s string) Game {
	game := Game{}

	parts := strings.Split(s, ":")
	id, err := strconv.Atoi(strings.Split(parts[0], " ")[1])
	if err != nil {
		log.Fatalf("Could not convert string to int while parsing game id in `newGame`: %v", err)
	}
	game.ID = id

	draws := strings.Split(parts[1], ";")
	for _, draw := range draws {
		game.Draws = append(game.Draws, newDraw(draw))
	}

	return game
}

func Part1(filepath string) int {
	input, err := os.ReadFile(filepath)
	if err != nil {
		log.Fatalf("Could not read the input file: %v", err)
	}

	sum := 0
	lines := strings.Split(string(input), "\n")
	mainDraw := Draw{Red: 12, Blue: 14, Green: 13}

	for _, line := range lines {
		game := newGame(line)
		minDraw := game.minDraw()
		if minDraw.canBeAccomodatedWith(&mainDraw) {
			sum += game.ID
		}
	}

	return sum
}
