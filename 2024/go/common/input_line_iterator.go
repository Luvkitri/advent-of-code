package common

import (
	"bufio"
	"os"
)

type LineIterator struct {
	file *os.File
	scanner *bufio.Scanner
	err error
}

func NewLineIterator(filePath string) (*LineIterator, error) {
	file, err := os.Open(filePath)

	if err != nil {return nil, err}

	return &LineIterator{
		file: file,
		scanner: bufio.NewScanner(file),
		err: nil,
	}, nil
}

func (iterator *LineIterator) Next() (string, bool) {
	if iterator.err != nil || !iterator.scanner.Scan() {
		if err := iterator.scanner.Err(); err != nil {
			iterator.err = err
		}
		iterator.Close()
		return "", true
	}
	return iterator.scanner.Text(), false
}

func (iterator *LineIterator) Close() {
	if iterator.file != nil {
		iterator.file.Close()
		iterator.file = nil
	}
}

