#include "matrix.h"

Matrix::Matrix(int nRows, int nCols, vector<Extensor> values)
{
	this->nRows = nRows;
	this->nCols = nCols;

	data = vector<tuple<int, int, Extensor>>();
	for (int i = 0; i < values.size(); ++i) {
		auto val = values[i];
		if (!val.is_zero()) {
			int row_index = i / nCols;
			int col_index = i % nCols;
			data.push_back(tuple<int, int, Extensor> {row_index, col_index, val});
		}
	}
}

vector<Extensor> Matrix::operator*(const vector<Extensor>& other)
{
	vector<Extensor> res(other.size(), Extensor::zero());

	for (const auto& triple : data) {
		int x = get<0>(triple);
		int y = get<1>(triple);
		Extensor v = get<2>(triple);
		res[x] = res[x] + v * other[y];
	}

	return res;
}