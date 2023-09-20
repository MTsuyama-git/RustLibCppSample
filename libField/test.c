#include <loadField.h>

int main(int argc, char** argv)
{
    Load(1);
    Load(1 << 31);
    Load(1023 << 1);
    Load(1023 << 11);
    Load(1023 << 21);
    return 0;
}