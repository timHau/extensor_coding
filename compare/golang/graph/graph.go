package graph

import (
	"bufio"
	"math/rand"
	"os"
	"strconv"
	"strings"

	"github.com/timHau/extensor_coding/extensor"
	"github.com/timHau/extensor_coding/matrix"
)

type Graph struct {
	AdjMat *matrix.Matrix
}

func createBernoulli(n int, k int) []*extensor.Extensor {
	res := make([]*extensor.Extensor, n)

	for i := 1; i <= n; i++ {
		coeffs := make([]int, k)
		basis := make([][]uint8, k)
		for j := 0; j < k; j++ {
			r := rand.Intn(2)
			if r == 0 {
				r = -1
			}
			coeffs = append(coeffs, r)
			basis = append(basis, []uint8{uint8(j) + 1})
		}
		e := extensor.New(coeffs, basis)
		res = append(res, e)
	}

	return res
}

func FromTsvWithCoding(path string, k int) *Graph {
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	trimmed := strings.Replace(lines[1], "% ", "", -1)
	splited := strings.Split(trimmed, " ")

	nrows, err := strconv.Atoi(splited[1])
	if err != nil {
		panic(err)
	}

	ncols, err := strconv.Atoi(splited[2])
	if err != nil {
		panic(err)
	}

	adjMat := make([]uint8, nrows*ncols)
	for _, line := range lines {
		if !strings.HasPrefix(line, "%") {
			values := strings.Split(line, " ")

			from, err := strconv.Atoi(values[0])
			if err != nil {
				panic(err)
			}

			to, err := strconv.Atoi(values[1])
			if err != nil {
				panic(err)
			}

			from = from - 1
			to = to - 1
			adjMat[from*ncols+to] = 1
		}
	}

	return NewWithCoding(k, nrows, ncols, adjMat)
}

func NewWithCoding(k int, nrows int, ncols int, data []uint8) *Graph {
	coding := createBernoulli(nrows*ncols, k)

	adjMat := make([]*extensor.Extensor, len(data))
	for i, v := range data {
		if v != 0 {
			rowIndex := i / ncols
			adjMat = append(adjMat, coding[rowIndex])
		}
	}

	return &Graph{
		AdjMat: matrix.New(nrows, ncols, adjMat),
	}
}

func (g *Graph) ComputeWalkSum(k int) *extensor.Extensor {
	b := make([]*extensor.Extensor, g.AdjMat.NumCols)

	b = g.AdjMat.Mul(b)
	for i := 1; i < k-1; i++ {
		b = g.AdjMat.Mul(b)
	}

	res := extensor.Zero()
	for _, e := range b {
		res.Add(e)
	}

	return res
}
