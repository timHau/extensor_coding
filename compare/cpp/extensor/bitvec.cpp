#include "bitvec.h"

Extensor::Extensor(vector<int> coeffs, vector<vector<int>> basis)
{
	assert(coeffs.size() == basis.size());

	data = map<bitset<32>, int, Comparer> {};
	for (int i = 0; i < basis.size(); ++i)
	{
		bitset<32> base;
		for (int j = 0; j < basis[i].size(); ++j)
		{
			base = base.set(basis[i][j], 1);
		}

		data[base] = coeffs[i];
	}
}

Extensor Extensor::operator + (Extensor const &other) {
	map<bitset<32>, int, Comparer> data;
	data = this->data;

	for (auto const& [key, val] : other.data)
	{
		if (data.find(key) == data.end())
		{
			data[key] = val;
		} else
		{
			data[key] += val;
		}
	}

	for (auto const& [key, val] : data) {
		std::cout << key << " : " << val << std::endl;
	}

	return Extensor(data);
}