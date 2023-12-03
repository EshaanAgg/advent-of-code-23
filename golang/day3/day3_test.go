package day3

import (
	"fmt"
	"testing"
)

type TestCase struct {
	Day         int
	Part        int
	Input       string
	Expected    int
	TestingFunc func(string) int
}

func TestDay1(t *testing.T) {

	testcases := []TestCase{
		{3, 1, "../../data/3/test.txt", 4361, Part1},
		{3, 2, "../../data/3/test.txt", 467835, Part2},
	}

	for _, testcase := range testcases {
		t.Run(fmt.Sprintf("[Day-%v][Part-%v]", testcase.Day, testcase.Part), func(t *testing.T) {
			if res := testcase.TestingFunc(testcase.Input); res != testcase.Expected {
				t.Errorf("Wrong solution for [Day %v][Part %v]; Expected = %v, Got = %v", testcase.Day, testcase.Part, testcase.Expected, res)
			}
		})
	}
}
