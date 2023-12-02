package day2

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
		{2, 1, "../../data/2/test.txt", 8, Part1},
		{2, 2, "../../data/2/test.txt", 2286, Part2},
	}

	for _, testcase := range testcases {
		t.Run(fmt.Sprintf("[Day-%v][Part-%v]", testcase.Day, testcase.Part), func(t *testing.T) {
			if res := testcase.TestingFunc(testcase.Input); res != testcase.Expected {
				t.Errorf("Wrong solution for [Day %v][Part %v]; Expected = %v, Got = %v", testcase.Day, testcase.Part, testcase.Expected, res)
			}
		})
	}
}
