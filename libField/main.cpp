#include <loadField.h>
#include <cstdio>

int main(int argc, char** argv)
{
    Register reg;
    reg.LSB = 1;
    Load(reg);
    reg.raw = 0;
    reg.MSB = 1;
    Load(reg);
    reg.raw = 0;
    reg.P0 = 1023;
    Load(reg);
    reg.raw = 0;
    reg.P1 = 1023;
    Load(reg);
    reg.raw = 0;
    reg.P2 = 1023;
    Load(reg);
    return 0;
}