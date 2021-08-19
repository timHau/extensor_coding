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

func TestMul2(t *testing.T) {
	vec := []extensor.Extensor{
		*extensor.New([]int{9}, [][]uint8{{1}}),
		*extensor.New([]int{8}, [][]uint8{{3}}),
	}
	m := matrix.New(2, 2, []*extensor.Extensor{
		extensor.New([]int{1}, [][]uint8{{1, 2}}),
		extensor.New([]int{2}, [][]uint8{{3, 4}}),
		extensor.New([]int{3}, [][]uint8{{1, 2}}),
		extensor.New([]int{4}, [][]uint8{{3, 4}}),
	})
	res := m.Mul(vec)

	if !(len(res) == 2) {
		t.Error("length should match")
	}

	if !res[0].IsZero() && !res[1].IsZero() {
		t.Error("should be zero")
	}
}

func TestMul3(t *testing.T) {
	vec := []extensor.Extensor{
		*extensor.New([]int{1, -1}, [][]uint8{{1}, {2}}),
		*extensor.New([]int{-1, 1}, [][]uint8{{1}, {2}}),
	}
	m := matrix.New(2, 2, []*extensor.Extensor{
		&(vec[0]),
		&(vec[0]),
		&(vec[1]),
		&(vec[1]),
	})
	res := m.Mul(vec)

	if !(len(res) == 2) {
		t.Error("length should match")
	}

	if !res[0].IsZero() && !res[1].IsZero() {
		t.Log(res)
		t.Error("should be zero")
	}
}

func TestMul4(t *testing.T) {
	vec := []extensor.Extensor{
		*extensor.New([]int{1, -1}, [][]uint8{{1}, {2}}).Lift(4),
		*extensor.New([]int{-1, 1}, [][]uint8{{1}, {2}}).Lift(4),
	}
	m := matrix.New(2, 2, []*extensor.Extensor{
		&(vec[0]),
		&(vec[0]),
		&(vec[1]),
		&(vec[1]),
	})
	res := m.Mul(vec)

	if !(len(res) == 2) {
		t.Error("length should match")
	}

	if !res[0].IsZero() && !res[1].IsZero() {
		t.Log(res)
		t.Error("should be zero")
	}
}
