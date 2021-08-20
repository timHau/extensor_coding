#include <iostream>
#include <vector>
#include "./extensor/bitvec.h"

using std::vector;

int main()
{
	Extensor e1(vector<int>{1}, vector<vector<int>>{{1}});
	Extensor e2(vector<int>{2}, vector<vector<int>>{{2}});

	e1 + e2;

	return 0;
}