#include <iostream>
#include <vector>
#include <math.h>
#include <chrono>
#include <fstream>

#include "./extensor.h"
#include "./matrix.h"
#include "./graph.h"

using std::vector;
using std::chrono::high_resolution_clock;
using std::chrono::duration_cast;
using std::chrono::duration;
using std::chrono::milliseconds;

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
	float eps = 0.8;
	int num_iter = 5;
	int max_k = 9;
	vector<float> times;

	for (int k = 2; k < max_k; k++) {
		vector<float> times_per_run;

		for (int i = 0; i < num_iter; i++) {
			auto t1 = high_resolution_clock::now();

			auto t = Graph::from_tsv_with_coding("out.brunson_revolution_revolution", k);
			Graph g = get<0>(t);
			float _res = algorithm_c(g, k, eps);

			auto t2 = high_resolution_clock::now();
			duration<float, std::milli> ms_elapsed = t2 - t1;

			times_per_run.push_back(ms_elapsed.count());
		}

		float mean_time = mean(times_per_run);
		times.push_back(mean_time);
	}

	std::ofstream file;
	file.open("bench_k_cpp.txt");

	int k = 2;
	for (const auto& t : times) {
		file << k << ", " << t << std::endl;
		k++;
	}
	file.close();

	return 0;
}