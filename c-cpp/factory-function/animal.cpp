#include <cstdio>
using namespace std;

enum class AnimalKind {
    Dog, Cat,
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
    case AnimalKind::Dog:
        return new Dog();
    case AnimalKind::Cat:
    default:
        return new Cat();
    }
}

int main() {
    auto d = Animal::choice(AnimalKind::Dog);
    d->say();

    auto c = Animal::choice(AnimalKind::Cat);
    c->say();

    delete d;
    delete c;

    return 0;
}
