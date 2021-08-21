#include <iostream>
#include <vector>
#include <math.h>

#include "./extensor.h"
#include "./matrix.h"
#include "./graph.h"

using std::vector;

int factorial(int n) {
	int res = 1;

	for (int i = 1; i <= n; i++) {
		res *= i;
	}

	return res;
}

float mean(vector<float> values) {
	float res = 0.0;

	for (const auto& value : values) {
		res += value;
	}

	return res / float(values.size());
}

float std_dev(vector<float> values) {
	float mean_val = mean(values);
	float res = 0.0;

	for (const auto& value : values) {
		res += pow(value - mean_val, 2);
	}

	return sqrt(res / float(values.size()));
}

float t_value(int degrees_of_freedom) {
	if (degrees_of_freedom <= 4) {
		return 3.747;
	}
	if (degrees_of_freedom <= 8) {
		return 2.896;
	}
	if (degrees_of_freedom <= 16) {
		return 2.583;
	}
	if (degrees_of_freedom <= 32) {
		return 2.457;
	}
	if (degrees_of_freedom <= 64) {
		return 2.390;
	}
	if (degrees_of_freedom <= 128) {
		return 2.358;
	}
	return 2.326;
}

float algorithm_c(Graph g, int k, float eps) {
	int step = 1;
	int mean_val = std::numeric_limits<float>::infinity();
	vector<float> values;
	vector<float> means;

	while (step < pow(k, 2) / pow(eps, 2)) {
		auto t = Graph::from_tsv_with_coding("out.brunson_revolution_revolution", k);
		Graph g = get<0>(t);
		vector<Extensor> coding = get<1>(t);
		Extensor v_j = g.compute_walk_sum(k, coding);
		int coeff;
		if (v_j.coeffs().size() == 0) {
			coeff = 0;
		} else {
			coeff = v_j.coeffs()[0];
		}

		int denom = float(factorial(k));
		float x_j = float(coeff) / denom;
		values.push_back(x_j);

		mean_val = mean(values);
		means.push_back(mean_val);
		float std_dev_val = std_dev(means);

		std::cout << "std_dev: " << std_dev_val << " mean: " << mean_val << " step: " << step << std::endl;
		float t_val = t_value(step - 1);
		if ((((mean_val - t_val * std_dev_val / sqrt(step)) > (1.0 - eps) * mean_val) || std_dev_val == 0.0 ) && step > 30) {
			return mean_val;
		}
		step++;
	}

	return mean_val;
}

int main(int argc, char *argv[])
{
	float eps = 0.5;

	if (argc == 1) {
		int k = 4;
		auto t = Graph::from_tsv_with_coding("out.brunson_revolution_revolution", k);
		Graph g = get<0>(t);
		float res = algorithm_c(g, k, eps);
		std::cout << "res: " << res << std::endl;
	}

	if (argc == 2) {
		int k = std::stoi(argv[1]);
		auto t = Graph::from_tsv_with_coding("out.brunson_revolution_revolution", k);
		Graph g = get<0>(t);
		float res = algorithm_c(g, k, eps);
		std::cout << "res: " << res << std::endl;
	}

	return 0;
}