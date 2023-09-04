#pragma once

inline static void* malloc(unsigned long size) {
    void* po6_rt_malloc(unsigned long size);

    return po6_rt_malloc(size);
}
inline static void* calloc(unsigned long a, unsigned long b) {
    void* po6_rt_calloc(unsigned long a, unsigned long b);

    return po6_rt_calloc(a, b);
}
inline static void free(void* ptr) {
    void po6_rt_free(void* ptr);

    return po6_rt_free(ptr);
}
inline static void* memalign(unsigned long alignment, unsigned long size) {
    void* po6_rt_memalign(unsigned long alignment, unsigned long size);

    return po6_rt_memalign(alignment, size);
}
inline static void* realloc(void *ptr, unsigned long size) {
    void* po6_rt_realloc(void *ptr, unsigned long size);

    return po6_rt_realloc(ptr, size);
}
