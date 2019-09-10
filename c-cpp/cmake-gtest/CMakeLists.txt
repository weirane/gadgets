cmake_minimum_required(VERSION 3.12)
project(cmake-gtest)

include(GoogleTest)
function(add_gtest target)
    add_executable(${target}
        ${target}.cpp
        ${target}_test.cpp
    )
    target_link_libraries(${target}
        gtest
        gtest_main
    )
    gtest_discover_tests(${target})
endfunction()

add_custom_target(run
    COMMAND ${PROJECT_NAME}
    DEPENDS ${PROJECT_NAME}
)

enable_testing()

add_gtest(factorial)
add_executable(${PROJECT_NAME}
    main.cpp
    factorial.cpp
)
