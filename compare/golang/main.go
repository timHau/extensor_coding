package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/timHau/extensor_coding/algorithm"
)

func main() {

	args := os.Args

	if len(args) > 1 {
		k, err := strconv.Atoi(os.Args[1])
		if err != nil {
			panic(err)
		}
		res := algorithm.C("../../src/data/out.brunson_revolution_revolution", k, 0.1)
		fmt.Println(res)
	}
}
