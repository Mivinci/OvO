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
    COMMAND ${CMAKE_COMMAND} -E make_directory
      ${VENDOR_WEBKIT_DIR}/lib
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
      ${WEBKIT_LIB_DIR}/JavaScriptCore.framework/Headers/*h
      ${VENDOR_WEBKIT_DIR}/include/JavaScriptCore/
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
      ${WEBKIT_LIB_DIR}/JavaScriptCore.framework/PrivateHeaders/*h
      ${VENDOR_WEBKIT_DIR}/include/JavaScriptCore/
    COMMAND ${CMAKE_COMMAND} -E copy_directory
      ${WEBKIT_LIB_DIR}/usr/local/include/wtf
      ${VENDOR_WEBKIT_DIR}/include/wtf
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
      ${WEBKIT_LIB_DIR}/libJavaScriptCore.a
      ${VENDOR_WEBKIT_DIR}/lib/
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
      ${WEBKIT_LIB_DIR}/libWTF.a
      ${VENDOR_WEBKIT_DIR}/lib/
  )
else()
  message(FATAL_ERROR "Systems other than Apple are not supported yet")
endif(APPLE)
