#ifndef CPP_SPARSE_TRIPLES_H
#define CPP_SPARSE_TRIPLES_H

#include <tuple>
#include "./extensor.h"

using std::vector, std::tuple;

class Matrix
{
private:
	int nRows;
	int nCols;
	vector<tuple<int, int, Extensor>> data;

public:
	Matrix(int nRows, int nCols, vector<Extensor> values);

	vector<Extensor> operator * (const vector<Extensor> &other);
};


#endif //CPP_SPARSE_TRIPLES_H
