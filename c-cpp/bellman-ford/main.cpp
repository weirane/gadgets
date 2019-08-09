#include <iostream>
#include "graph.h"

using namespace std;

auto bellman_ford(const Graph &graph, int start, vector<double> &dist,
                  vector<int> &prev, bool &neg_cycle) -> bool;

int main() {
    auto g = Graph::with_node_count(4);
    g.add_edge(0, 1, 1);
    g.add_edge(0, 2, 3);
    g.add_edge(1, 2, 1);
    g.add_edge(2, 0, 2);

    auto dist = vector<double>{};
    auto prev = vector<int>{};
    bool neg_cycle;

    if (!bellman_ford(g, 0, dist, prev, neg_cycle)) {
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
