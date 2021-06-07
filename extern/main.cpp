#include <core.hpp>

typedef uint64_t u64;
typedef uint32_t u32;
typedef uint16_t u16;

core::RAM _RAM;

struct WatchDog
{
    u16 stctrlh;
    u16 stctrll;
};
