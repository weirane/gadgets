#include <gtest/gtest.h>
#include "factorial.h"

TEST(factorial, zero) {
    EXPECT_EQ(1, factorial(0));
}

TEST(factorial, negative) {
    EXPECT_EQ(1, factorial(-100));
}

TEST(factorial, normal) {
    EXPECT_EQ(6, factorial(3));
    EXPECT_EQ(362880, factorial(9));
}
