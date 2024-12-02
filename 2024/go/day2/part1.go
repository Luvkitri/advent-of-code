package day2

import (
	"aoc2024/common"
	"log"
	"strconv"
	"strings"
)

type SortOrder int

const (
	Ascending SortOrder = iota
	Descending
	Constant
)

func SolvePart1() int {
	var fileName = "day2/puzzle-input.txt"
	reports := ParseInput(fileName)

	
	var count = 0
	for _, report := range reports {
		if checkReport1(report) {
			count += 1
		}
	}

	return count
}

func checkReport1(report []int) bool {
	var isCorrect = true
	var prevOrder = Constant
	var order = Constant
	var j = 1
	for i := 0; i < len(report) - 1; i++ {
		first := report[i]
		second := report[j]
		diff := first - second

		if diff < 0 {
			order = Ascending 
		}

		if diff > 0 {
			order = Descending
		}

		absDiff := common.AbsInt(diff)
		if (order != Constant && prevOrder != Constant && order != prevOrder) || absDiff < 1 || absDiff > 3{
			isCorrect = false
			break;
		}
		prevOrder = order	
		j++
	}

	return isCorrect
}

func ParseInput(fileName string) [][]int {
	iterator, err := common.NewLineIterator(fileName)

	if err != nil {
		log.Fatal(err)
	}

	var reports [][]int
	
	for {
		line, isEOF := iterator.Next()
		if isEOF { break }
		report_raw:= strings.Split(line, " ")
		report := make([]int, len(report_raw))
		for i, level := range report_raw {
			report[i], _ = strconv.Atoi(level)
		}
		reports = append(reports, report)
		
	}

	return reports
}
