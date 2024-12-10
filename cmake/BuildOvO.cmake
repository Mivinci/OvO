set(ovo "OvO")

file(GLOB OVO_SOURCES
  ${CMAKE_SOURCE_DIR}/src/binding/*.c
  ${CMAKE_SOURCE_DIR}/src/binding/*.c++
)

add_library(${ovo} STATIC ${OVO_SOURCES})

target_include_directories(${ovo} PRIVATE
  ${CMAKE_SOURCE_DIR}/src/binding
  ${VENDOR_WEBKIT_DIR}/include
)

target_link_libraries(${ovo} PRIVATE
  ${WEBKIT_LIB_DIR}/libWTF.a
  ${WEBKIT_LIB_DIR}/libJavaScriptCore.a
)

set_target_properties(${ovo} PROPERTIES
  C_STANDARD 99
  C_STANDARD_REQUIRED ON
  CXX_STANDARD 20
  CXX_STANDARD_REQUIRED ON
)

# add_dependencies(${ovo} ${jsc})
