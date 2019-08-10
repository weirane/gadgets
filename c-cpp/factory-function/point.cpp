#include <iostream>
#include <cmath>
using namespace std;

class Point {
    double x, y;
    Point(double _x, double _y) : x(_x), y(_y) {}

public:
    static auto cartesian(double x, double y) -> Point {
        return Point(x, y);
    }

    static auto polar(double r, double theta) -> Point {
        return Point(r * cos(theta), r * sin(theta));
    }

    friend auto operator<<(ostream &os, const Point &stuff) -> ostream & {
        return os << "Point {x = " << stuff.x << ", "
                  << "y = " << stuff.y << "}";
    }
};

int main() {
    auto p1 = Point::cartesian(3, 4);
    cout << "p1 = " << p1 << endl;

    auto p2 = Point::polar(5, atan(4.0 / 3.0));
    cout << "p2 = " << p2 << endl;

    return 0;
}
