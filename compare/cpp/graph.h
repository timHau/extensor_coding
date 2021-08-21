#ifndef CPP_GRAPH_H
#define CPP_GRAPH_H

#include "matrix.h"
#include <string>
#include <fstream>
#include <sstream>
#include <iostream>
#include <iterator>
#include <iomanip>
#include <stdlib.h>
#include <time.h>

using std::string, std::vector;

class Graph
{
private:
	Matrix adj_mat;
	int num_verts;

public:
	Graph(Matrix m, int verts) : adj_mat(m), num_verts(verts) {};
	static std::tuple<Graph, vector<Extensor>> from_tsv_with_coding(string path_str, int k);
	static vector<Extensor> create_bernoulli(int n, int k);
	Extensor compute_walk_sum(int k, vector<Extensor> coding);
};


#endif //CPP_GRAPH_H
