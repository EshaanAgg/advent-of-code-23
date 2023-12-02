package main

import (
	"fmt"
	"golang/day2"
)

type Solver struct{}

type TestCase struct {
	Day         int
	Part        int
	Input       string
	Expected    int
	TestingFunc func(string) int
}

func main() {
	fmt.Printf("Day 2 Part 1: %v\n", day2.Part1("../data/2/input.txt"))
	fmt.Printf("Day 2 Part 2: %v\n", day2.Part2("../data/2/input.txt"))
}
