#include <vector>
#include <limits>

class Edge {
    Edge(int from, int to, double wei) : src(from), dst(to), weight(wei) {}

public:
    int src, dst;
    double weight;

    static auto with_weight(int from, int to, int weight) {
        return Edge(from, to, weight);
    }

    static auto max_weighed(int from, int to) {
        return Edge(from, to, std::numeric_limits<double>::infinity());
    }
};

class Node {
    Node() {}

public:
    std::vector<Edge> adj;

    static auto empty() {
        return Node();
    }
};

class GraphEdgeIter;
class Graph {
    Graph() {}

public:
    std::vector<Node> nodes;

    static auto empty() {
        return Graph();
    }

    static auto with_node_count(int n) {
        auto g = Graph();
        for (int i = 0; i < n; i++)
            g.add_node();
        return g;
    }

    auto add_edge(int from, int to, double weight) -> bool;

    auto add_edge_both_way(int from, int to, double weight) -> bool {
        if (this->node_count() <= from || this->node_count() <= to)
            return false;
        this->add_edge(from, to, weight);
        this->add_edge(to, from, weight);
        return true;
    }

    auto add_node() -> void {
        this->nodes.push_back(Node::empty());
    }

    auto node_count() const -> int {
        return this->nodes.size();
    }

    auto clear() -> void {
        this->nodes.clear();
    }

    auto clear_edges() -> void {
        for (auto &node : this->nodes)
            node.adj.clear();
    }

    auto edges() const -> GraphEdgeIter;
};

class GraphEdgeIter {
    using NodeIter = std::vector<Node>::const_iterator;
    using EdgeIter = std::vector<Edge>::const_iterator;

    const Graph &the_graph;
    NodeIter node_it;
    EdgeIter edge_it;

    GraphEdgeIter(const Graph &g, NodeIter nit, EdgeIter eit)
        : the_graph(g), node_it(nit), edge_it(eit) {}

public:
    static auto fresh(const Graph &the_graph) {
        auto node_begin = the_graph.nodes.cbegin();
        return GraphEdgeIter(the_graph, node_begin, node_begin->adj.cbegin());
    }

    auto begin() -> GraphEdgeIter {
        auto node_begin = this->the_graph.nodes.cbegin();
        return GraphEdgeIter(this->the_graph, node_begin,
                             node_begin->adj.cbegin());
    }

    auto end() -> GraphEdgeIter {
        auto node_end = this->the_graph.nodes.cend();
        return GraphEdgeIter(this->the_graph, node_end, node_end->adj.cbegin());
    }

    auto operator!=(const GraphEdgeIter &rhs) -> bool {
        return this->edge_it != rhs.edge_it || this->node_it != rhs.node_it;
    }

    auto operator*() -> const Edge & {
        return *this->edge_it;
    }

    auto operator++() -> GraphEdgeIter &;
};
