set(bindings "JavaScriptCore-CXX-Bindings")

file(GLOB CXX_SOURCES
  ${CMAKE_SOURCE_DIR}/src/*.c
  ${CMAKE_SOURCE_DIR}/src/*.c++
)

add_library(${bindings} STATIC ${CXX_SOURCES})

target_include_directories(${bindings} PUBLIC
  ${CMAKE_SOURCE_DIR}/src
  ${VENDOR_WEBKIT_DIR}/include
  ${WEBKIT_BUILD_DIR}/usr/local/include
)

target_link_libraries(${bindings} PUBLIC
  "-framework JavaScriptCore"
)

set_target_properties(${bindings} PROPERTIES
  C_STANDARD 99
  C_STANDARD_REQUIRED ON
  CXX_STANDARD 20
  CXX_STANDARD_REQUIRED ON
)

add_dependencies(${bindings} ${jsc})
