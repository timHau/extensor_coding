package matrix

import (
	"github.com/timHau/extensor_coding/extensor"
)

type Triple struct {
	rowIndex int
	colIndex int
	Value    *extensor.Extensor
}

type Matrix struct {
	NumRows int
	NumCols int
	Data    []*Triple
}

func New(NumRows int, NumCols int, Values []*extensor.Extensor) *Matrix {
	data := []*Triple{}

	for i, v := range Values {
		if !v.IsZero() {
			rowIndex := i / NumCols
			colIndex := i % NumCols
			data = append(data, &Triple{
				rowIndex: rowIndex,
				colIndex: colIndex,
				Value:    Values[i],
			})
		}
	}

	return &Matrix{
		NumRows: NumRows,
		NumCols: NumCols,
		Data:    data,
	}
}

func (m *Matrix) Mul(other []extensor.Extensor) []extensor.Extensor {
	data := make([]extensor.Extensor, m.NumRows)

	for _, v := range m.Data {
		val := *v.Value.Mul(other[v.colIndex])
		data[v.rowIndex] = *data[v.rowIndex].Add(val)
	}

	return data
}

func (m *Matrix) Get(i int, j int) extensor.Extensor {
	for _, triple := range m.Data {
		if triple.rowIndex == i && triple.colIndex == j {
			return *triple.Value
		}
	}

	return *extensor.Zero()
}

func (m *Matrix) Set(i int, j int, e *extensor.Extensor) {
	for _, triple := range m.Data {
		if triple.rowIndex == i && triple.colIndex == j {
			triple.Value = e
		}
	}
}
