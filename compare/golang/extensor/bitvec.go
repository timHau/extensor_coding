package extensor

import (
	"github.com/teivah/bitvector"
)

type Extensor struct {
	Data map[bitvector.Len32]int
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

func GetIndices(bv *bitvector.Len32) []int {
	indices := []int{}

	for i, v := range bv.String() {
		if v == '1' {
			indices = append(indices, i)
		}
	}

	return indices
}

func GetSign(a *bitvector.Len32, b *bitvector.Len32) int {
	numPerm := 0

	indicesA := GetIndices(a)
	indicesB := GetIndices(b)

	i := 0
	j := 0
	for i < len(indicesA) && j < len(indicesB) {
		if indicesA[i] >= indicesB[j] {
			i += 1
		} else {
			j += 1
			numPerm += int(a.Count()) - i
		}
	}

	if numPerm%2 == 0 {
		return 1
	}
	return -1
}

func New(coeffs []int, basis [][]uint8) *Extensor {
	if len(coeffs) != len(basis) {
		panic("coefficients and basis dimensions should match")
	}

	data := make(map[bitvector.Len32]int)

	for i, b := range basis {
		data[BitvecFromBasis(b)] = coeffs[i]
	}

	return &Extensor{
		Data: data,
	}
}

func (e *Extensor) Add(other *Extensor) *Extensor {
	data := make(map[bitvector.Len32]int)

	for b, c := range e.Data {
		data[b] = c
	}

	for b, c := range other.Data {
		if val, ok := data[b]; ok {
			data[b] = val + c
		} else {
			data[b] = c
		}
	}

	return &Extensor{
		Data: data,
	}
}

func (e *Extensor) Mul(other *Extensor) *Extensor {
	data := make(map[bitvector.Len32]int)

	for baseA, coeffA := range e.Data {
		for baseB, coeffB := range other.Data {
			if baseA.And(baseB).Count() == 0 {
				nextBase := baseA.Or(baseB)
				sign := GetSign(&baseA, &baseB)
				nextCoeff := sign * coeffA * coeffB

				if val, ok := data[nextBase]; ok {
					data[nextBase] = val + nextCoeff
				} else {
					data[nextBase] = nextCoeff
				}
			}
		}
	}

	return &Extensor{
		Data: data,
	}
}

func (e *Extensor) IsZero() bool {
	return len(e.Data) == 0
}

func Zero() *Extensor {
	return &Extensor{
		Data: make(map[bitvector.Len32]int),
	}
}

func (e *Extensor) Coeffs() []int {
	res := []int{}

	for _, v := range e.Data {
		res = append(res, v)
	}

	return res
}

func (e *Extensor) Lift(k uint8) *Extensor {
	data := make(map[bitvector.Len32]int)

	for b, c := range e.Data {
		b = b.Push(k)
		data[b] = c
	}

	return e.Mul(&Extensor{Data: data})
}
