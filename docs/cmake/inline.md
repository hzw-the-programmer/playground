https://stackoverflow.com/questions/2765164/inline-vs-inline-vs-inline-vs-forceinline
```
inline is the keyword, in C++ and C99.

__inline is a vendor-specific keyword (e.g. MSVC) for inline function in C, since C89 doesn't have it.

__inline__ is similar to __inline but is from another set of compilers.

__forceinline is another vendor-specific (mainly MSVC) keyword, which will apply more force to inline the function than the __inline hint (e.g. inline even if it result in worse code).

There's also __attribute__((always_inline)) in GCC and clang.
```

libevent/CMakeLists.txt
```
# Check for different inline keyword versions.
check_function_keywords("inline" "__inline" "__inline__")

if (HAVE_INLINE)
    set(EVENT__inline inline)
elseif (HAVE___INLINE)
    set(EVENT__inline __inline)
elseif(HAVE___INLINE__)
    set(EVENT__inline __inline__)
else()
    set(EVENT__inline)
endif()
```