#include "JavaScriptCore/Completion.h"
#include "JavaScriptCore/Exception.h"
#include "JavaScriptCore/JSCJSValue.h"
#include "JavaScriptCore/SourceCode.h"

using JSC__JSValue = JSC::JSValue;
using JSC__JSGlobalObject = JSC::JSGlobalObject;
using JSC__SourceCode = JSC::SourceCode;
using JSC__Exception = JSC::Exception;

void JSC__initialize(void);
JSC__JSValue JSC__evalute(JSC__JSGlobalObject *global_object,
                          const JSC__SourceCode &source_code,
                          JSC__JSValue this_value);
