#include "jsc.h"

#include "JavaScriptCore/APICast.h"
#include "JavaScriptCore/InitializeThreading.h"
#include "JavaScriptCore/JSCJSValue.h"
#include "JavaScriptCore/ThrowScope.h"


void JSC__initialize(void) {
  return JSC::initialize();
}

JSC__JSValue JSC__evalute(JSC__JSGlobalObject *global_object, const JSC__SourceCode& source_code, JSC__JSValue this_value) {
  NakedPtr<JSC::Exception> exception = nullptr;
  auto scope = DECLARE_THROW_SCOPE(global_object->vm());
  auto result = JSC::evaluate(global_object, source_code, this_value, exception);
  if (exception.get()) {
    scope.throwException(global_object, exception.get());
    return JSC::jsUndefined();
  }
  return result;
}
