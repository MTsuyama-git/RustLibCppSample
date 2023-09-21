#include <loadField.h>

int main(int argc, char** argv)
{
    uint32_t raw = 1;
    Load(&raw);
    raw = 1 << 31;
    Load(&raw);
    raw = 1023 << 1;
    Load(&raw);
    raw = 1023 << 11;
    Load(&raw);
    raw = 1023 << 21;
    Load(&raw);
    return 0;
}