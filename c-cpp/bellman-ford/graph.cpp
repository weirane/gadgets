#include <catch2/catch.hpp>
#include "graph.h"

using std::get;
using std::make_tuple;
using std::vector;

auto Graph::add_edge(int from, int to, double weight) -> bool {
    if (this->node_count() <= from || this->node_count() <= to)
        return false;

    this->nodes[from].adj.push_back(Edge::with_weight(from, to, weight));
    return true;
}

auto Graph::edges() const -> GraphEdgeIter {
    return GraphEdgeIter::fresh(*this);
}

auto GraphEdgeIter::operator++() -> GraphEdgeIter & {
    this->edge_it++;

    while (this->edge_it == this->node_it->adj.cend()) {
        this->node_it++;
        this->edge_it = this->node_it->adj.cbegin();
        if (this->node_it == this->the_graph.nodes.cend())
            return *this;
    }

    return *this;
}

TEST_CASE("Graph", "") {
    auto g = Graph::empty();

    REQUIRE(g.node_count() == 0);

    g.add_node();
    REQUIRE_FALSE(g.add_edge(0, 1, 89));

    REQUIRE(g.node_count() == 1);
}

TEST_CASE("Edge iterator", "") {
    auto g = Graph::with_node_count(5);
    REQUIRE(g.add_edge(2, 4, 3.));
    REQUIRE(g.add_edge(1, 2, 3.));
    REQUIRE(g.add_edge(3, 2, 3.));
    REQUIRE(g.add_edge(2, 1, 3.));
    REQUIRE(g.add_edge(0, 1, 3.));
    REQUIRE(g.add_edge(4, 0, 3.));

    auto iter_rslt = vector<std::tuple<int, int>>{};
    for (auto e : g.edges())
        iter_rslt.push_back(make_tuple(e.src, e.dst));

    auto expected = vector<std::tuple<int, int>>{
        make_tuple(0, 1), make_tuple(1, 2), make_tuple(2, 4),
        make_tuple(2, 1), make_tuple(3, 2), make_tuple(4, 0),
    };

    REQUIRE(iter_rslt == expected);
}
