#pragma once
#include <fieldDef.h>

#if defined(__cplusplus)
void Load(const Register& reg) noexcept;

extern "C"
{
#else
#include <stdint.h>
typedef uint32_t Register;
#endif
void Load(const void* pReg);

#if defined(__cplusplus)
}
#endif