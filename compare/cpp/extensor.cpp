#include "extensor.h"

Extensor::Extensor(vector<int> coeffs, vector<vector<int>> basis)
{
	assert(coeffs.size() == basis.size());

	data = map<bitset<32>, int, Comparer> {};
	for (int i = 0; i < basis.size(); ++i) {
		bitset<32> base;
		for (int j = 0; j < basis[i].size(); ++j) {
			base = base.set(basis[i][j], 1);
		}

		data[base] = coeffs[i];
	}
}

vector<int> Extensor::get_indices(bitset<32> bv)
{
	vector<int> res;

	for (int i = 0; i < bv.size(); ++i) {
		if (bv[i] == 1) {
			res.push_back(i);
		}
	}

	return res;
}

int Extensor::get_sign(bitset<32> b_1, bitset<32> b_2)
{
	int num_perm = 0;

	auto indices_a = get_indices(b_1);
	auto indices_b = get_indices(b_2);

	int i = 0, j = 0;
	while (i < indices_a.size() && j < indices_b.size()) {
		if (indices_a[i] <= indices_b[j]) {
			i++;
		} else {
			j++;
			num_perm += indices_a.size() - i;
		}
	}

	return num_perm % 2 == 0 ? 1 : -1;
}

void Extensor::lift(int k)
{
	map<bitset<32>, int, Comparer> lifted_data;

	for (const auto& [base, coeff] : this->data) {
		lifted_data[base << k] = coeff;
	}

	data = lifted_data;
}

bool Extensor::is_zero()
{
	return this->data.empty();
}

Extensor Extensor::zero()
{
	map<bitset<32>, int, Comparer> data;
	return Extensor{data};
}

Extensor Extensor::operator + (Extensor const &other) {
	map<bitset<32>, int, Comparer> data;
	data = this->data;

	for (const auto& [base, coeff] : other.data) {
		if (data.find(base) == data.end()) {
			data[base] = coeff;
		} else {
			data[base] += coeff;
		}
	}

	return Extensor(data);
}

Extensor Extensor::operator*(const Extensor& other)
{
	map<bitset<32>, int, Comparer> data;

	for (const auto& [base_a, coeff_a] : this->data) {
		for (const auto& [base_b, coeff_b] : other.data) {
			bitset<32> intersection = base_a & base_b;
			if (intersection.count() == 0) {
				bitset<32> next_base = base_a ^ base_b;
				int sign = get_sign(base_a, base_b);
				int next_coeff = sign * coeff_a * coeff_b;

				if (data.find(next_base) == data.end()) {
					data[next_base] = next_coeff;
				} else {
					data[next_base] += next_coeff;
				}
			}
		}
	}

	return Extensor(data);
}

void Extensor::debug()
{
	for (const auto& d : data) {
		std::cout << d.first << " : " << d.second << std::endl;
	}
}