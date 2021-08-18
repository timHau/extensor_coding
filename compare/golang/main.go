package main

import (
	"fmt"

	"github.com/timHau/extensor_coding/extensor"
	"github.com/timHau/extensor_coding/matrix"
)

func main() {
	m := matrix.New(2, 2, []int{1, 2, 3, 4})
	fmt.Println(m)

	e := extensor.New([]int{1, 2, 3}, [][]uint8{{1}, {2}, {3}})

	fmt.Println(e)
}
