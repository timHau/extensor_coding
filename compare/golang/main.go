package main

import (
	"bufio"
	"fmt"
	"os"
	"time"

	"github.com/timHau/extensor_coding/algorithm"
)

func main() {
	num_iter := 3
	times := []float64{}
	max_k := 8

	for k := 2; k <= max_k; k++ {
		timesPerRun := []float64{}

		fmt.Println(k)
		for i := 0; i < num_iter; i++ {
			start := time.Now()
			_ = algorithm.C("./out.brunson_revolution_revolution", k, 0.8)
			elapsed := time.Since(start)
			timesPerRun = append(timesPerRun, float64(elapsed.Milliseconds()))
		}

		m := algorithm.Mean(timesPerRun)
		times = append(times, m)
	}

	file, err := os.Create("./bench_k_golang.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	w := bufio.NewWriter(file)
	for i, t := range times {
		line := fmt.Sprintf("%v, %v", i+2, t)
		fmt.Fprintln(w, line)
	}
	w.Flush()
}
