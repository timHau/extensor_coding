package graph

import (
	"bufio"
	"fmt"
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

	for i := 0; i < n; i++ {
		coeffs := make([]int, k)
		basis := [][]uint8{}
		for j := 1; j <= k; j++ {
			r := rand.Intn(2)
			if r == 0 {
				r = -1
			}
			coeffs[j-1] = r
			basis = append(basis, []uint8{uint8(j)})
		}
		e := extensor.New(coeffs, basis)
		res[i] = e.Lift(k)
	}

	return res
}

func AdjMatFromTsv(path string) (int, int, []uint8) {
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

	return nrows, ncols, adjMat
}

func NewWithCoding(k int, nrows int, ncols int, data []uint8) (*Graph, []extensor.Extensor) {
	coding := createBernoulli(ncols, k)

	adjMat := make([]*extensor.Extensor, len(data))
	for i, v := range data {
		if v != 0 {
			rowIndex := i / ncols
			adjMat[i] = coding[rowIndex]
		}
	}

	coding_into := make([]extensor.Extensor, len(coding))
	for i, v := range coding {
		coding_into[i] = *v
	}

	return &Graph{
		AdjMat: matrix.New(nrows, ncols, adjMat),
	}, coding_into
}

func (g *Graph) ComputeWalkSum(k int, coding []extensor.Extensor) int {
	b := g.AdjMat.Mul(coding)
	for i := 1; i < k-1; i++ {
		b = g.AdjMat.Mul(b)
	}

	fmt.Println(coding)

	resExt := extensor.Zero()
	for _, e := range b {
		resExt = resExt.Add(e)
	}

	res := 0
	for _, c := range resExt.Coeffs() {
		res += c
	}

	return res
}
