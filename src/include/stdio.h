#pragma once

#define FILE void

#define stdin (FILE*)0
#define stdout (FILE*)1
#define stderr (FILE*)2

#define vfprintf(...) do {} while (0)
#define fprintf(...) do {} while (0)
#define printf(...) do {} while (0)

#define fopen(...) (FILE*)0
#define fwrite(...) do {} while (0)
#define fclose(...) do {} while (0)
