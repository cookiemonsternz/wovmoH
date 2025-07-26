#ifndef DATATYPES_H
#define DATATYPES_H

#include "lib/json.hpp" // For json
using json = nlohmann::json;

#include <variant>
#include <string>
#include <cmath>

enum class PinDataType
{
    INVALID,
    NUMBER,
    BOOLEAN,
    COLOR,
    VECTOR
};

class ColorData
{
public:
    float r = 0.0f, g = 0.0f, b = 0.0f, a = 0.0f;

    // INITIALIZATION
    ColorData() {};
    ColorData(float r, float g, float b, float a) : r(r), g(g), b(b), a(a) {};
    ColorData(float r, float g, float b) : r(r), g(g), b(b) {};
    ColorData(float x) : r(x), g(x), b(x) {};

    std::string toString() const
    {
        return std::to_string(r) + ", " + std::to_string(g) + ", " + std::to_string(b) + "," + std::to_string(a);
    }

    bool operator==(const ColorData &other) const
    {
        return r == other.r && g == other.g && b == other.b && a == other.a;
    }
    // COLORS
    ColorData operator+(const ColorData &other) const
    { // Color + Color
        return ColorData{r + other.r, g + other.g, b + other.b, a + other.a};
    }
    ColorData operator-(const ColorData &other) const
    { // Color - Color
        return ColorData{r - other.r, g - other.g, b - other.b, a - other.a};
    }
    ColorData operator*(const ColorData &other) const
    { // Color * Color
        return ColorData{r * other.r, g * other.g, b * other.b, a * other.a};
    }
    ColorData operator/(const ColorData &other) const
    { // Color / Color
        return ColorData{r / other.r, g / other.g, b / other.b, a / other.a};
    }
    // SCALARS
    ColorData operator+(float scalar) const
    { // Color + Scalar
        return ColorData{r + scalar, g + scalar, b + scalar, a + scalar};
    }
    ColorData operator-(float scalar) const
    { // Color - Scalar
        return ColorData{r - scalar, g - scalar, b - scalar, a - scalar};
    }
    ColorData operator*(float scalar) const
    { // Color * Scalar
        return ColorData{r * scalar, g * scalar, b * scalar, a * scalar};
    }
    ColorData operator/(float scalar) const
    { // Color / Scalar
        return ColorData{r / scalar, g / scalar, b / scalar, a / scalar};
    }
};

class VectorData
{
public:
    float x = 0.0f, y = 0.0f, z = 0.0f;

    // INITIALIZATION
    VectorData() {};
    VectorData(float x, float y, float z) : x(x), y(y), z(z) {};
    VectorData(float x, float y) : x(x), y(y) {};

    VectorData operator-() const { return VectorData(-x, -y, -z); }

    VectorData& operator+=(const VectorData &v) 
    {
        x += v.x;
        y += v.y;
        z += v.z;
        return *this;
    };

    VectorData& operator*=(float t) {
        x *= t;
        y *= t;
        z *= t;
        return *this;
    };

    VectorData& operator/=(float t) {
        return *this *= 1.0 / t;
    };

    float length() const 
    {
        return std::sqrt(length_squared());
    };

    float length_squared() const
    {
        return x * x + y * y + z * z;
    };

    // UTILITY
    std::string toString() const
    {
        return std::to_string(x) + ", " + std::to_string(y) + ", " + std::to_string(z);
    };

    // Serialization
    json to_json() const
    {
        json j;
        j["x"] = x;
        j["y"] = y;
        j["z"] = z;
        return j;
    };
};

inline VectorData operator+(const VectorData &u, const VectorData &v)
{ // Vector + Vector
    return VectorData(u.x + v.x, u.y + v.y, u.z + v.z);
}
inline VectorData operator-(const VectorData &u, const VectorData &v)
{ // Vector - Vector
    return VectorData(u.x - v.x, u.y - v.y, u.z - v.z);
}
inline VectorData operator*(const VectorData &u, const VectorData &v)
{ // Vector * Vector
    return VectorData(u.x * v.x, u.y * v.y, u.z * v.z);
}
inline VectorData operator/(const VectorData &u, const VectorData &v)
{ // Vector / Vector
    return VectorData(u.x / v.x, u.y / v.y, u.z / v.z);
}
// SCALARS
inline VectorData operator+(const VectorData &u, float t)
{ // Vector + Scalar
    return VectorData{u.x + t, u.y + t, u.z + t};
}
inline VectorData operator-(const VectorData &u, float t)
{ // Vector - Scalar
    return VectorData{u.x - t, u.y - t, u.z - t};
}
inline VectorData operator*(const VectorData &u, float t)
{ // Vector * Scalar
    return VectorData{u.x * t, u.y * t, u.z * t};
}
inline VectorData operator/(const VectorData &u, float t)
{ // Vector / Scalar
    return VectorData{u.x / t, u.y / t, u.z / t};
}
// BOOLS
inline bool operator==(const VectorData &u, const VectorData &v)
{
    return u.x == v.x && u.y == v.y && u.z == v.z;
}

using DataValue = std::variant<
    std::monostate, // Undefined / invalid
    double,         // Number
    bool,           // Bool
    ColorData,      // Color
    VectorData      // Vector
    >;

#endif // DATATYPES_H
