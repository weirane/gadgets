#include <catch2/catch.hpp>
#include "graph.h"

using std::get;
using std::make_tuple;
using std::vector;

auto bellman_ford(const Graph &graph, int start, std::vector<double> &dist,
                  std::vector<int> &prev, bool &neg_cycle) -> bool {
    auto node_cnt = graph.node_count();
    if (node_cnt <= start)
        return false;
    dist.resize(node_cnt);
    prev.resize(node_cnt);

    // Initialization
    for (auto &p : prev)
        p = -1;
    for (auto &d : dist)
        d = std::numeric_limits<double>::infinity();
    prev[start] = -1;
    dist[start] = 0;

    // Relax |V|-1 times
    for (int i = 0, count = graph.node_count(); i < count - 1; i++) {
        bool changed = false;
        for (auto &e : graph.edges()) {
            if (dist[e.src] + e.weight < dist[e.dst]) {
                dist[e.dst] = dist[e.src] + e.weight;
                prev[e.dst] = e.src;
                changed = true;
            }
        }
        if (!changed)
            goto FINISH;
    }

    // Check for negative cycles
    for (auto &e : graph.edges()) {
        if (dist[e.src] + e.weight < dist[e.dst]) {
            neg_cycle = true;
            return true;
        }
    }

FINISH:
    neg_cycle = false;
    return true;
}

TEST_CASE("Bellman Ford", "") {
    auto g = Graph::with_node_count(5);
    auto dist = vector<double>{};
    auto prev = vector<int>{};
    bool neg_cycle = false;
    bool good = true;

    if (neg_cycle == good) {
        // Used to eliminate warnings
    }

    REQUIRE(g.node_count() == 5);

    SECTION("A normal case - 1") {
        REQUIRE(g.add_edge(0, 1, 5));
        REQUIRE(g.add_edge(0, 2, 3));
        REQUIRE(g.add_edge(2, 1, -3));

        REQUIRE(bellman_ford(g, 0, dist, prev, neg_cycle));
        REQUIRE_FALSE(neg_cycle);

        auto dist_expected = vector<double>{0, 0, 3};
        auto prev_expected = vector<int>{-1, 2, 0};

        REQUIRE(vector<double>(dist.begin(), dist.begin() + 3)
                == dist_expected);
        REQUIRE(vector<int>(prev.begin(), prev.begin() + 3) == prev_expected);
    }

    g.clear_edges();
    REQUIRE(g.node_count() == 5);

    SECTION("A normal case - 2") {
        auto edges = {
            make_tuple(0, 1, -1.), make_tuple(0, 2, 4.),  make_tuple(1, 2, 3.),
            make_tuple(1, 3, 2.),  make_tuple(1, 4, 2.),  make_tuple(3, 2, 5.),
            make_tuple(3, 1, 1.),  make_tuple(4, 3, -3.),
        };
        for (const auto &e : edges) {
            if (!g.add_edge(get<0>(e), get<1>(e), get<2>(e)))
                good = false;
        }
        REQUIRE(good);

        REQUIRE(bellman_ford(g, 0, dist, prev, neg_cycle));
        REQUIRE_FALSE(neg_cycle);

        auto dist_expected = vector<double>{0, -1, 2, -2, 1};
        auto prev_expected = vector<int>{-1, 0, 1, 4, 1};
        REQUIRE(dist == dist_expected);
        REQUIRE(prev == prev_expected);
    }

    g.clear_edges();

    SECTION("With negative circle") {
        auto edges = {
            make_tuple(0, 1, 3.),
            make_tuple(0, 2, 2.),
            make_tuple(1, 2, 1.),
            make_tuple(2, 0, -5.),
        };
        for (const auto &e : edges) {
            if (!g.add_edge(get<0>(e), get<1>(e), get<2>(e)))
                good = false;
        }
        REQUIRE(good);

        REQUIRE(bellman_ford(g, 0, dist, prev, neg_cycle));
        REQUIRE(neg_cycle);
    }

    SECTION("Out of range error") {
        REQUIRE_FALSE(bellman_ford(g, 100, dist, prev, neg_cycle));
    }
}
