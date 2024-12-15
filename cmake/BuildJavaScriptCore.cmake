set(jsc "JavaScriptCore")
set(BUILD_OPTIONS)

if(DEBUG)
  list(APPEND BUILD_OPTIONS "--debug")
endif(DEBUG)

set(VENDOR_WEBKIT_DIR ${VENDOR_DIR}/WebKit)

if(APPLE)
  add_custom_target(${jsc} COMMAND ${WEBKIT_DIR}/Tools/Scripts/build-jsc ${BUILD_OPTIONS})
  add_custom_command(TARGET ${jsc} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory
      ${VENDOR_WEBKIT_DIR}/include/JavaScriptCore
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
      ${WEBKIT_BUILD_DIR}/JavaScriptCore.framework/Headers/*h
      ${VENDOR_WEBKIT_DIR}/include/JavaScriptCore/
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
      ${WEBKIT_BUILD_DIR}/JavaScriptCore.framework/PrivateHeaders/*h
      ${VENDOR_WEBKIT_DIR}/include/JavaScriptCore
  )
else()
  message(FATAL_ERROR "Platforms other than Apple are not supported yet")
endif(APPLE)
