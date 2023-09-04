#pragma once

#define NULL ((void*)0)

inline static void memcpy(void* dest, const void* src, unsigned long n) {
    void po6_rt_memcpy(void* dest, const void* src, unsigned long n);

    return po6_rt_memcpy(dest, src, n);
}
inline static void memset(void *str, int c, unsigned long n) {
    void po6_rt_memset(void *str, int c, unsigned long n);

    return po6_rt_memset(str, c, n);
}
inline static int memcmp(const void* s1, const void* s2, unsigned long n) {
    int po6_rt_memcmp(const void* s1, const void* s2, unsigned long n);

    return po6_rt_memcmp(s1, s2, n);
}
