#include <core.hpp>

typedef uint64_t u64;
typedef uint32_t u32;
typedef uint16_t u16;

core::RAM _RAM;

struct WatchDog
{
    size_t address;
    WatchDog() : address(address){};
    u16 stctrlh;
    u16 stctrll;

    void Disable(){};
};

int main()
{
    auto x = core::newObject<WatchDog>(0x00);
}
