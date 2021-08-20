#include <iostream>
#include <vector>
#include "./extensor/bitvec.h"

using std::vector;

int main()
{
	Extensor e1(vector<int>{1}, vector<vector<int>>{{1}});
	Extensor e2(vector<int>{2}, vector<vector<int>>{{5}});

	e1 * e2;

	e1.lift(9);
	for (const auto& [base, coeff] : e1.data) {
		std::cout << base << std::endl;
	}

	return 0;
}