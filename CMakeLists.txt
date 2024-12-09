cmake_minimum_required(VERSION 3.24)
project(OvO VERSION 0.0.1)

list(APPEND CMAKE_MODULE_PATH
  ${CMAKE_SOURCE_DIR}/cmake
)

include(SetupGlobal)
include(SetupWebKit)

include(BuildJSC)
include(BuildOvO)
