#include <iostream>
#include <fstream>
#include "graph.h"

using namespace std;

auto bellman_ford(const Graph &graph, int start, vector<double> &dist,
                  vector<int> &prev, bool &neg_cycle) -> bool;

int main(int argc, char *argv[]) {
    ifstream data = argc > 1 ? ifstream(argv[1]) : ifstream("data");
    if (!data.good()) {
        cout << "Error open data file" << endl;
        return 1;
    }

    int node_cnt, start;
    if (!(data >> node_cnt >> start)) {
        cout << "Error while reading data file" << endl;
        return 1;
    }

    auto g = Graph::with_node_count(node_cnt);
    int from, to;
    double weight;
    while (data >> from >> to >> weight) {
        g.add_edge(from, to, weight);
    }

    auto dist = vector<double>{};
    auto prev = vector<int>{};
    bool neg_cycle;

    if (!bellman_ford(g, start, dist, prev, neg_cycle)) {
        cout << "Error occurs" << endl;
        return 1;
    }

    if (neg_cycle) {
        cout << "Has negative cycle" << endl;
        return 0;
    }

    cout << "Distance: ";
    for (auto d : dist)
        cout << d << ' ';
    cout << endl;

    cout << "Prev node: ";
    for (auto p : prev)
        cout << p << ' ';
    cout << endl;

    return 0;
}
