package day2

import "aoc2024/common"

func SolvePart2() int {
	var fileName = "day2/test-input.txt"
	reports := ParseInput(fileName)

	
	var count = 0
	for _, report := range reports {
		if checkReport2(report) {
			count += 1
		}
	}

	return count
}

func checkReport2(report []int) bool {
	var count = 0
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
		if (order != Constant && prevOrder != Constant && order != prevOrder) || absDiff < 1 || absDiff > 3 {
			count += 1
		}

		if count > 1 {
			return false
		}

		prevOrder = order	
		j++
	}

	return true
}
