
#include <iostream>
#include <string>

struct Point {
    int x;
    int y;
};

class Circle {
private:
    Point center;
    double radius;

public:
    Circle(int x, int y, double r) : center{x, y}, radius(r) {}

    double getArea() const {
        return 3.14159 * radius * radius;
    }
};

void printCircleInfo(const Circle& c) {
    std::cout << "Circle area: " << c.getArea() << std::endl;
}

int main() {
    Circle myCircle(0, 0, 5);
    printCircleInfo(myCircle);
    return 0;
}
