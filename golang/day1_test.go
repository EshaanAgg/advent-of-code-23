package main

import (
	"fmt"
	"testing"
)

func TestDay1(t *testing.T) {
	s := Solver{}

	testcases := []TestCase{
		{1, 1, "../data/1/test1.txt", 142, s.Day1Part1},
		{1, 2, "../data/1/test2.txt", 281, s.Day1Part2},
	}

	for _, testcase := range testcases {
		t.Run(fmt.Sprintf("[Day-%v][Part-%v]", testcase.Day, testcase.Part), func(t *testing.T) {
			if res := testcase.TestingFunc(testcase.Input); res != testcase.Expected {
				t.Errorf("Wrong solution for [Day %v][Part %v]; Expected = %v, Got = %v", testcase.Day, testcase.Part, testcase.Expected, res)
			}
		})
	}
}
