#pragma once

void po6_rt_assert_fail(const char* expr, const char* file, int line);

#define assert(condition) do { \
    if ((condition) == 0) po6_rt_assert_fail(#condition, __FILE__, __LINE__); \
} while (0)
