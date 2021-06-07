#include <bits/stdc++.h>

namespace core
{
    struct RAM
    {
        std::unordered_map<size_t, size_t> memory;
    };

    size_t read_volatile(size_t address, RAM ram)
    {
        return ram.memory[address];
    }

    void write_volatile(size_t value, size_t address, RAM ram)
    {
        ram.memory[address] = value;
    }

    void _nop()
    {
        std::cout << "Stopping for 1 clock cycle!";
    }
} // namespace core
