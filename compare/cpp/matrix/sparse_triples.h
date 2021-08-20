#ifndef CPP_SPARSE_TRIPLES_H
#define CPP_SPARSE_TRIPLES_H

#include <tuple>

using std::vector, std::tuple;

class sparse_triples
{
private:
	size_t nRows;
	size_t nCols;
	vector<tuple<int, int>> data;
};


#endif //CPP_SPARSE_TRIPLES_H
