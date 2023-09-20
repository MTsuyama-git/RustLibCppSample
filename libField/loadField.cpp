#include <fieldDef.h>
#include <iostream>

#include <loadField.h>

void Load(const Register& reg) noexcept
{
    printf("RAW:0x%08X\n", reg.raw);
    printf("\tLSB:0x%08X\n", static_cast<uint32_t>(reg.LSB));
    printf("\t P0:0x%08X\n", static_cast<uint32_t>(reg.P0));
    printf("\t P1:0x%08X\n", static_cast<uint32_t>(reg.P1));
    printf("\t P2:0x%08X\n", static_cast<uint32_t>(reg.P2));
    printf("\tMSB:0x%08X\n", static_cast<uint32_t>(reg.MSB));
    return;
}

void Load(const uint32_t reg)
{
    Register __reg;
    __reg.raw = reg;
    Load(__reg);
}