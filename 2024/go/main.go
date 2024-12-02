package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
	"aoc2024/day2"
)

func main() {
	fmt.Println(day2.SolvePart1())
	fmt.Println(day2.SolvePart2())
}

func task1() {
	
	fmt.Println("AOC Task 1")
	first_column, second_column := read_file_input()
	sort.Sort(sort.IntSlice(first_column))
	sort.Sort(sort.IntSlice(second_column))

	distance_sum := 0.0
	for i := range(first_column) {
		distance_sum += math.Abs(float64(first_column[i]) - float64(second_column[i]))
	}

	fmt.Println(int(distance_sum))

	first_column_map := make(map[int]int)
	for _, value := range first_column {
		first_column_map[value] = 0
	}

	for _, value := range second_column {

		count, ok := first_column_map[value]
		if ok {
			first_column_map[value] = count + 1
		}
	}

	sim_score := 0
	for key, count := range first_column_map {
		sim_score += key * count 
	}
	
	fmt.Println(sim_score)
}

func read_file_input() ([]int, []int) {
	file, err := os.Open("task-input.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	
	reader := bufio.NewReader(file)

	var first_column []int
	var second_column []int

	for {
		line, _, err := reader.ReadLine()
		if err != nil {
			if err.Error() == "EOF" {
				break
			}
		log.Fatal(err)
		}
		lineStr := string(line)

		substrings := strings.Split(lineStr, "   ")

		value, err := strconv.Atoi(substrings[0])		
		if err != nil {
			log.Fatal(err)
		}
		first_column = append(first_column, value)

		value, err = strconv.Atoi(substrings[1])		
		if err != nil {
			log.Fatal(err)
		}
		second_column = append(second_column, value)
	}

	return first_column, second_column
}
