package main

import (
	"reflect"
	"testing"

	"github.com/timHau/extensor_coding/extensor"
	"github.com/timHau/extensor_coding/matrix"
)

func TestMul(t *testing.T) {
	vec := []extensor.Extensor{
		*extensor.New([]int{5}, [][]uint8{{3}}),
		*extensor.New([]int{6}, [][]uint8{{4}}),
	}
	m := matrix.New(2, 2, []*extensor.Extensor{
		extensor.New([]int{1}, [][]uint8{{1}}),
		extensor.New([]int{2}, [][]uint8{{2}}),
		extensor.New([]int{3}, [][]uint8{{5}}),
		extensor.New([]int{4}, [][]uint8{{6}}),
	})
	res := m.Mul(vec)
	expect := []extensor.Extensor{
		*extensor.New([]int{5, 12}, [][]uint8{{1, 3}, {2, 4}}),
		*extensor.New([]int{-15, -24}, [][]uint8{{3, 5}, {4, 6}}),
	}

	for i, val := range res {
		if !reflect.DeepEqual(val, expect[i]) {
			t.Log(res)
			t.Log(expect)
			t.Error("TEST")
		}
	}
}
