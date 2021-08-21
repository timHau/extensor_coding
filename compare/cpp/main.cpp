#include <iostream>
#include <vector>

#include "./extensor.h"
#include "./matrix.h"
#include "./graph.h"

using std::vector;

int main()
{
	Extensor e1(vector<int>{1}, vector<vector<int>>{{1}});
	Extensor e2(vector<int>{2}, vector<vector<int>>{{2}});
	Extensor e3(vector<int>{3}, vector<vector<int>>{{3}});
	Extensor e4(vector<int>{4}, vector<vector<int>>{{4}});

	Matrix M(2, 2, vector<Extensor>{ e1, e2, e3, e4});

	Extensor e5(vector<int>{5}, vector<vector<int>>{{5}});
	Extensor e6(vector<int>{6}, vector<vector<int>>{{6}});

	vector<Extensor> v{e5, e6};

	int k = 3;
	auto t = Graph::from_tsv_with_coding("out.brunson_revolution_revolution", k);
	Graph g = get<0>(t);
	vector<Extensor> coding = get<1>(t);

	return 0;
}