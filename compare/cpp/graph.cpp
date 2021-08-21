#include "graph.h"

std::tuple<Graph, vector<Extensor>> Graph::from_tsv_with_coding(string path_str, int k)
{
	std::ifstream file(path_str);
	string line;

	vector<string> lines;
	while (std::getline(file, line)) {
		lines.push_back(line);
	}

	std::istringstream iss(lines[1]);
	vector<string> dim{std::istream_iterator<string>(iss), {}};
	int nrows = std::stoi(dim[2]);
	int ncols = std::stoi(dim[3]);

	auto coding = create_bernoulli(nrows + ncols, k);

	vector<Extensor> adj_mat_data(nrows * ncols, Extensor::zero());
	for (const auto& line : lines) {
		if (line.rfind("%", 0) != 0) { // lines does not start with %
			std::istringstream is(line);
			vector<string> values{std::istream_iterator<string>(is), {}}; // split whitespace

			int from = std::stoi(values[0]) - 1;
			int to = std::stoi(values[1]) - 1;
			adj_mat_data[from * ncols + to] = coding[from];
		}
	}

	Matrix adj_mat = Matrix(nrows, ncols, adj_mat_data);
	Graph g{adj_mat, nrows + ncols};
	return std::make_tuple(g, coding);
}

vector<Extensor> Graph::create_bernoulli(int n, int k)
{
	vector<Extensor> res(n, Extensor::zero());

	srand(time(NULL)); // init random seed
	for (int i = 0; i < n; ++i) {
		vector<int> coeffs(k);
		vector<vector<int>> base(k);
		for (int j = 0; j < k; ++j) {
			int r = rand() % 2;
			if (r == 0) {
				r = -1;
			}
			coeffs[j] = r;
			base[j] = vector<int>{j+1};
		}
		Extensor e{coeffs, base};
		res[i] = e.lift(k);
	}

	return res;
}

Extensor Graph::compute_walk_sum(int k, vector<Extensor> coding)
{
	vector<Extensor> b = this->adj_mat * coding;

	for (int i = 1; i < k-1; ++i) {
		b = this->adj_mat * b;
	}

	Extensor res = Extensor::zero();
	for (const auto& e : b) {
		res = res + e;
	}

	return res;
}