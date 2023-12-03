package main

import (
	"fmt"
	"golang/day3"
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
	fmt.Printf("Day 3 Part 1: %v\n", day3.Part1("../data/3/input.txt"))
	fmt.Printf("Day 3 Part 2: %v\n", day3.Part2("../data/3/input.txt"))
}
