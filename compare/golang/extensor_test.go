package main

import (
	"reflect"
	"testing"

	"github.com/teivah/bitvector"
	"github.com/timHau/extensor_coding/extensor"
)

func TestAdd(t *testing.T) {
	a := extensor.New([]int{2, 5}, [][]uint8{{1, 3}, {3, 9}})
	b := extensor.New([]int{1, 1}, [][]uint8{{1, 2}, {3, 9}})
	c := a.Add(*b)
	res := extensor.New([]int{1, 2, 6}, [][]uint8{{1, 2}, {1, 3}, {3, 9}})

	if !reflect.DeepEqual(c.Data, res.Data) {
		t.Error("should add")
	}
}

func TestAdd2(t *testing.T) {
	a := extensor.New([]int{-3, 4}, [][]uint8{{1, 3}, {3, 9}})
	b := extensor.New([]int{3, -4}, [][]uint8{{1, 3}, {3, 9}})
	c := a.Add(*b)
	res := extensor.New([]int{0, 0}, [][]uint8{{1, 3}, {3, 9}})

	if !reflect.DeepEqual(c.Data, res.Data) {
		t.Error("should add")
	}
}

func TestSign(t *testing.T) {
	var x_1 bitvector.Len32
	x_1 = x_1.Set(2, true)
	var x_2 bitvector.Len32
	x_2 = x_2.Set(2, true)
	if extensor.GetSign(&x_1, &x_2) != 1 {
		t.Error("sign should be 1")
	}
}

func TestSign3(t *testing.T) {
	var x_1 bitvector.Len32
	x_1 = x_1.Set(1, true)
	var x_2 bitvector.Len32
	x_2 = x_2.Set(3, true)
	if extensor.GetSign(&x_2, &x_1) != -1 {
		t.Error("sign should be -1")
	}
}

func TestSign4(t *testing.T) {
	var x_1 bitvector.Len32
	x_1 = x_1.Set(1, true)
	x_1 = x_1.Set(2, true)
	x_1 = x_1.Set(4, true)
	var x_2 bitvector.Len32
	x_2 = x_2.Set(3, true)
	x_2 = x_2.Set(5, true)
	x_2 = x_2.Set(6, true)
	if extensor.GetSign(&x_2, &x_1) != -1 {
		t.Error("sign should be -1")
	}
}

func TestVanish(t *testing.T) {
	x_1 := extensor.New([]int{1}, [][]uint8{{1}})
	p := x_1.Mul(*x_1)
	if !p.IsZero() {
		t.Error("Should be zero")
	}
}

func TestMul1(t *testing.T) {
	x_1 := extensor.New([]int{3}, [][]uint8{{3, 4}})
	x_2 := extensor.New([]int{4}, [][]uint8{{2, 6}})
	res := x_1.Mul(*x_2)
	expect := extensor.New([]int{12}, [][]uint8{{2, 3, 4, 6}})
	if !reflect.DeepEqual(res.Data, expect.Data) {
		t.Log(x_1.Data)
		t.Log(res.Data)
		t.Error("should match")
	}
}

func TestAntiComm(t *testing.T) {
	x_1 := extensor.New([]int{2}, [][]uint8{{1}})
	x_2 := extensor.New([]int{4}, [][]uint8{{3}})
	p_1 := x_1.Mul(*x_2)
	res_1 := extensor.New([]int{8}, [][]uint8{{1, 3}})
	if !reflect.DeepEqual(p_1.Data, res_1.Data) {
		t.Log(p_1.Data)
		t.Log(res_1.Data)
		t.Error("should match")
	}

	p_2 := x_2.Mul(*x_1)
	res_2 := extensor.New([]int{-8}, [][]uint8{{1, 3}})
	if !reflect.DeepEqual(p_2.Data, res_2.Data) {
		t.Log(p_2.Data)
		t.Log(res_2.Data)
		t.Error("should match")
	}
}

func TestLift(t *testing.T) {
	x_1 := extensor.New([]int{2, 3}, [][]uint8{{1}, {2}})
	lifted := x_1.Lift(2)
	x_2 := extensor.New([]int{2, 3}, [][]uint8{{3}, {4}})
	res := x_1.Mul(*x_2)

	if !reflect.DeepEqual(res.Data, lifted.Data) {
		t.Log(res.Data)
		t.Log(lifted.Data)
		t.Error("lift should work")
	}
}

func TestMulTmp(t *testing.T) {
	var bv1 bitvector.Len32
	bv1 = bv1.Set(1, true)
	bv1 = bv1.Set(5, true)

	var bv2 bitvector.Len32
	bv2 = bv2.Set(2, true)
	bv2 = bv2.Set(5, true)

	var bv3 bitvector.Len32
	bv3 = bv3.Set(1, true)
	bv3 = bv3.Set(6, true)

	var bv4 bitvector.Len32
	bv4 = bv4.Set(2, true)
	bv4 = bv4.Set(6, true)

	e_1 := extensor.Extensor{
		Data: map[bitvector.Len32]int{
			bv1: 1,
			bv2: -1,
			bv3: -1,
			bv4: 1,
		},
	}

	if extensor.GetSign(&bv4, &bv1) != -1 {
		t.Log(e_1.Mul(e_1))
		t.Error("sign is wrong")
	}
}
