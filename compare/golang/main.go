package main

import (
	"fmt"

	"github.com/timHau/extensor_coding/extensor"
)

func main() {
	e := extensor.New([]int{1, 2, 3}, [][]uint8{{1}, {2}, {3}})

	fmt.Println(e)
}
