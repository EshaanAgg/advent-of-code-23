package main

import "fmt"

type Solver struct{}

type TestCase struct {
	Day         int
	Part        int
	Input       string
	Expected    int
	TestingFunc func(string) int
}

func main() {
	s := Solver{}

	fmt.Printf("Day 1 Part 1: %v\n", s.Day1Part1("../data/1/input.txt"))
	fmt.Printf("Day 1 Part 2: %v\n", s.Day1Part2("../data/1/input.txt"))
}
