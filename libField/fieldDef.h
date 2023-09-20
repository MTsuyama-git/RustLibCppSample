#pragma once
#if defined(__cplusplus)
#include <cstddef>
#include <cstdint>
#include <cassert>
template <typename RegType, size_t Width, size_t Offset>
struct VariableField
{
    RegType m_Reg;
    static constexpr RegType Max = ((static_cast<RegType>(1) << (Width)) - 1);
    static_assert(sizeof(RegType) * 8 >= (Width + Offset));
    RegType operator=(RegType value)
    {
        assert(value <=  Max);
        m_Reg &= ~(Max << Offset);
        m_Reg |= (value << Offset);
        return m_Reg;
    }

    inline operator RegType() const
    {
        return (m_Reg>> Offset) & Max;
    }

    

};

template <size_t Width, size_t Offset>
using Field8 = VariableField<std::uint8_t, Width, Offset>;
template <size_t Width, size_t Offset>
using Field16 = VariableField<std::uint16_t, Width, Offset>;
template <size_t Width, size_t Offset>
using Field32 = VariableField<std::uint32_t, Width, Offset>;
template <size_t Width, size_t Offset>
using Field64 = VariableField<std::uint64_t, Width, Offset>;

typedef union
{
    Field32<1, 0>   LSB;
    Field32<10, 1>   P0;
    Field32<10, 11>  P1;
    Field32<10, 21>  P2;
    Field32<1, 31>  MSB;
    uint32_t raw;
} Register;

#endif