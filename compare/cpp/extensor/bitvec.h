#ifndef CPP_BITVEC_H
#define CPP_BITVEC_H

#include <map>
#include <bitset>
#include <iostream>
#include <vector>
#include "assert.h"

using std::vector, std::bitset, std::map;

struct Comparer {
	bool operator() (const bitset<32> &b1, const bitset<32> &b2) const {
		return b1.to_ulong() < b2.to_ulong();
	}
};

class Extensor
{
private:
	map<bitset<32>, int, Comparer> data;

public:
	Extensor(vector<int> coeffs, vector<vector<int>> basis);
	Extensor(map<bitset<32>, int, Comparer> init_data) : data(init_data) {};

	Extensor operator + (Extensor const &other);
};


#endif //CPP_BITVEC_H
