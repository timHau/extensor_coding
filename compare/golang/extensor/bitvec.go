package extensor

import "github.com/teivah/bitvector"

type Extensor struct {
	data map[bitvector.Len32]int
}

func BitvecFromBasis(basis []uint8) bitvector.Len32 {
	var b bitvector.Len32

	for _, pos := range basis {
		if pos >= 32 {
			panic("Basis index is to large")
		}
		b = b.Set(pos, true)
	}

	return b
}

func NewExtensor(coeffs []int, basis [][]uint8) *Extensor {
	if len(coeffs) != len(basis) {
		panic("coefficients and basis dimensions should match")
	}

	data := make(map[bitvector.Len32]int)

	for i, b := range basis {
		data[BitvecFromBasis(b)] = coeffs[i]
	}

	return &Extensor{
		data: data,
	}
}

func (e *Extensor) Add(other *Extensor) *Extensor {
	data := make(map[bitvector.Len32]int)

	for b, c := range e.data {
		data[b] = c
	}

	for b, c := range other.data {
		if val, ok := data[b]; ok {
			data[b] = val + c
		}
	}

	return &Extensor{
		data: data,
	}
}
