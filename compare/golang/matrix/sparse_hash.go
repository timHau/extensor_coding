package matrix

type Matrix struct {
	NumRows int
	NumCols int
	Data    []int
}

func New(NumRows int, NumCols int, Data []int) *Matrix {
	return &Matrix{
		NumRows: NumRows,
		NumCols: NumCols,
		Data:    Data,
	}
}
