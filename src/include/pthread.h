#pragma once

typedef struct {} pthread_t;
typedef struct {} pthread_attr_t;
typedef struct {} pthread_once_t;
typedef struct {} pthread_cond_t;
typedef struct {} pthread_mutex_t;

#define PTHREAD_ONCE_INIT {}

#define pthread_create(...) 0
#define pthread_join(...) 0
#define pthread_attr_init(...) 0
#define pthread_once_init(...) 0
#define pthread_cond_init(...) 0
#define pthread_mutex_init(...) 0
#define pthread_attr_destroy(...) 0
#define pthread_once_destroy(...) 0
#define pthread_cond_destroy(...) 0
#define pthread_mutex_destroy(...) 0
#define pthread_mutex_lock(...) 0
#define pthread_mutex_unlock(...) 0
#define pthread_cond_wait(...) 0
#define pthread_cond_signal(...) 0
#define pthread_cond_broadcast(...) 0
#define pthread_once(...) 0
#define pthread_attr_setstacksize(...) 0
#define pthread_setname_np(...) 0
