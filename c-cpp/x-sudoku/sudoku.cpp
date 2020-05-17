#include <cstdio>
#include <cstdlib>
#include <functional>

#include <iostream>
#include <unordered_set>

using namespace std;

class Board {
public:
    int square[9][9] = {{0}};
    bool possible[9][9][10] = {{{true}}};
    int blank_count = 81;

    void place(int x, int y, int val) {
        this->square[x][y] = val;
        for (int k = 0; k < 9; k++) {
            this->possible[x][k][val] = false;
            this->possible[k][y][val] = false;
        }
        if (x == y) {
            for (int k = 0; k < 9; k++)
                this->possible[k][k][val] = false;
        }
        if (x + y == 8) {
            for (int k = 0; k < 9; k++)
                this->possible[k][8 - k][val] = false;
        }
        for (int k = 0; k < 9; k++) {
            this->possible[k / 3 + x / 3 * 3][k % 3 + y / 3 * 3][val] = false;
        }
        for (int k = 0; k <= 9; k++)
            this->possible[x][y][val] = k == val;
        this->blank_count--;
    }

    static auto from_iter(function<int(int, int)> gen) {
        Board b;
        for (int i = 0; i < 9; i++)
            for (int j = 0; j < 9; j++)
                for (int k = 0; k < 10; k++)
                    b.possible[i][j][k] = true;

        for (int i = 0; i < 9; i++) {
            for (int j = 0; j < 9; j++) {
                int v = gen(i, j);
                if (v != 0) {
                    b.place(i, j, v);
                }
            }
        }
        return b;
    }

    static auto from_array(int a[][9]) {
        return Board::from_iter([=](int i, int j) { return a[i][j]; });
    }

    static auto from_input() {
        return Board::from_iter([](int i, int j) {
            int v = 0;
            int n = scanf("%d", &v);
            if (n != 1) {
                fprintf(stderr, "invalid input on %d:%d\n", i + 1, j + 1);
                exit(1);
            }
            return v;
        });
    }

    auto pos(int p) const {
        return (p >= 0 && p < 81) ? this->square[p / 9][p % 9] : -1;
    }

    void print() const {
        for (int i = 0; i < 9; i++) {
            for (int j = 0; j < 9; j++) {
                printf("%d%c", this->square[i][j], j == 8 ? '\n' : ' ');
            }
        }
    }

#define CHECK(s, n)                     \
    do {                                \
        if ((n) == 0)                   \
            continue;                   \
        if ((n) < 0 || (n) > 9)         \
            return false;               \
        if ((s).find((n)) != (s).end()) \
            return false;               \
        (s).insert((n));                \
    } while (false)

    bool is_valid() const {
        unordered_set<int> s1;
        unordered_set<int> s2;
        s1.reserve(9);
        s2.reserve(9);
        int c = this->blank_count;
        for (int i = 0; i < 9; i++) {
            s1.clear();
            s2.clear();
            for (int j = 0; j < 9; j++) {
                // rows
                auto n = this->square[i][j];
                CHECK(s1, n);
                // columns
                auto n2 = this->square[j][i];
                CHECK(s2, n2);
                // blank_count
                if (this->square[i][j] != 0) {
                    c++;
                }
            }
        }
        if (c != 81) {
            return false;
        }
        s1.clear();
        s2.clear();
        for (int i = 0; i < 9; i++) {
            // ⟍
            auto n = this->square[i][i];
            CHECK(s1, n);
            // ⟋
            auto n2 = this->square[8 - i][i];
            CHECK(s2, n2);
        }
        // 3x3
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                s1.clear();
                for (int k = 0; k < 9; k++) {
                    auto n = this->square[k / 3 + i * 3][k % 3 + j * 3];
                    CHECK(s1, n);
                }
            }
        }
        return true;
    }
#undef CHECK

    // Checks whether val can be placed at (x, y).
    bool can_place(int x, int y, int val) const {
        // full
        if (this->blank_count <= 0)
            return false;
        // invalid place
        if (x < 0 || x > 8 || y < 0 || y > 8)
            return false;
        // occupied
        if (this->square[x][y] != 0)
            return false;
        // invalid value
        if (val < 1 || val > 9)
            return false;
        return this->possible[x][y][val];
    }

#define FORWARD_CHECK(a, b)                        \
    do {                                           \
        int __x = (a);                             \
        int __y = (b);                             \
        if (this->square[__x][__y] == 0) {         \
            int poss = -1, count = 0;              \
            for (int j = 1; j <= 9; j++) {         \
                if (this->possible[__x][__y][j]) { \
                    poss = j;                      \
                    count++;                       \
                }                                  \
            }                                      \
            if (count == 0) {                      \
                return false;                      \
            } else if (count == 1) {               \
                this->place(__x, __y, poss);       \
            }                                      \
        }                                          \
    } while (false)

    /// Checks whether cells has at least one possible value after placing at
    /// [x, y]. Also place the value if it's the only possible one.
    bool forward_check(int x, int y) {
        for (int i = 0; i < 9; i++) {
            // row, column
            FORWARD_CHECK(x, i);
            FORWARD_CHECK(i, y);
        }
        // x
        if (x == y) {
            for (int i = 0; i < 9; i++) {
                FORWARD_CHECK(i, i);
            }
        }
        if (x + y == 8) {
            for (int i = 0; i < 9; i++) {
                FORWARD_CHECK(i, 8 - i);
            }
        }
        return true;
    }
#undef FORWARD_CHECK

    bool solve() {
        function<bool(int)> back_track = [&](int pos) {
            while (pos < 81 && this->pos(pos) != 0)
                pos++;
            if (pos == 81) {
                return true;
            }
            for (int i = 1; i <= 9; i++) {
                int x = pos / 9;
                int y = pos % 9;
                if (this->can_place(x, y, i)) {
                    Board bak = *this;
                    this->place(x, y, i);
                    if (this->forward_check(x, y) && back_track(pos + 1)) {
                        return true;
                    }
                    *this = bak;
                }
            }
            return false;
        };
        return back_track(0);
    }
};

ostream &operator<<(ostream &os, const Board &b) {
    os << "Board {{\n";
    for (int i = 0; i < 9; i++) {
        os << "    ";
        for (int j = 0; j < 9; j++) {
            os << b.square[i][j] << (j == 8 ? "\n" : " ");
            if (j == 2 || j == 5) {
                os << "│ ";
            }
        }
        if (i == 2 || i == 5) {
            os << "    ──────┼───────┼──────\n";
        }
    }
    os << "}, " << b.blank_count << "}";
    return os;
}

int main(void) {
    auto s = Board::from_input();
    s.solve();
    s.print();
}
