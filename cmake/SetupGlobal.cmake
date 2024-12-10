if(CMAKE_BUILD_TYPE STREQUAL "Debug") 
  set(BUILD_TYPE "Debug")
  set(DEBUG ON)
else()
  set(BUILD_TYPE "Release")
  set(DEBUG OFF)
endif()

set(VENDOR_DIR ${CMAKE_SOURCE_DIR}/vendor)
