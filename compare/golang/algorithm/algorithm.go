package algorithm

import (
	"fmt"
	"math"

	"github.com/timHau/extensor_coding/graph"
)

func Factorial(n int) int {
	val := 1
	for i := 1; i <= n; i++ {
		val *= i
	}
	return val
}

func Mean(values []float64) float64 {
	val := 0.0
	for _, v := range values {
		val += v
	}
	return val / float64(len(values))
}

func StdDev(values []float64) float64 {
	mean := Mean(values)
	val := 0.0

	for _, v := range values {
		val += math.Pow(v-mean, 2.0)
	}

	return math.Sqrt(val / float64(len(values)))
}

func TValue(degOfFreedom uint32) float64 {
	if degOfFreedom <= 4 {
		return 3.747
	}
	if degOfFreedom <= 8 {
		return 2.896
	}
	if degOfFreedom <= 16 {
		return 2.583
	}
	if degOfFreedom <= 32 {
		return 2.457
	}
	if degOfFreedom <= 64 {
		return 2.390
	}
	if degOfFreedom <= 128 {
		return 2.358
	}
	return 2.326
}

func C(graphPath string, k int, eps float64) float64 {
	step := 1
	mean := math.Inf(1)
	values := []float64{}
	means := []float64{}

	nrows, ncols, adjMat := graph.AdjMatFromTsv(graphPath)
	for step < int(math.Pow(float64(k), 2.0)/math.Pow(eps, 2.0)) {
		g, coding := graph.NewWithCoding(k, nrows, ncols, adjMat)
		v_j := g.ComputeWalkSum(k, coding)
		denom := float64(Factorial(k))
		x_j := math.Abs(float64(v_j)) / denom
		fmt.Println(v_j)
		values = append(values, x_j)

		mean = Mean(values)
		means = append(means, mean)
		stdDev := StdDev(means)
		// tVal := TValue(uint32(step - 1))

		fmt.Printf("stdDev: %v, mean: %v, step: %v \n", stdDev, mean, step)

		/*
			if mean-tVal*stdDev/math.Sqrt(float64(step)) > (1.0-eps)*mean {
				return mean
			}
		*/
		step += 1
	}

	return mean
}
