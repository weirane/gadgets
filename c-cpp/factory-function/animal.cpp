#include <cstdio>
using namespace std;

enum AnimalKind {
    dog, cat,
};

class Animal {
public:
    static auto choice(AnimalKind kind) -> Animal*;

    virtual auto say() -> void = 0;
    virtual ~Animal() {};
};

class Dog: public Animal {
public:
    Dog() {};

    auto say() -> void {
        printf("I am a dog.\n");
    }
};

class Cat: public Animal {
public:
    Cat() {};

    auto say() -> void {
        printf("I am a cat.\n");
    }
};

auto Animal::choice(AnimalKind kind) -> Animal* {
    switch(kind) {
    case AnimalKind::dog:
        return new Dog();
    case AnimalKind::cat:
    default:
        return new Cat();
    }
}

int main() {
    auto d = Animal::choice(AnimalKind::dog);
    d->say();

    auto c = Animal::choice(AnimalKind::cat);
    c->say();

    delete d;
    delete c;

    return 0;
}
