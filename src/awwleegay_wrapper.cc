#include <nan.h>
#include <ctime>
#include "platform.h"

extern "C"
{
    char* rust_hello();
    void rust_capitalize(char *);
    int rust_add(int, int);
    void rust_free_string(char *);
    void rust_free_vec_string(char*[2]);
    char* awwleegay_parse_xlog_to_file(char *,char *);
    char* awwleegay_parse_xlog_to_string(char *);
    char** awwleegay_parse_xlog_to_file_tmp(char *);
}


void Hello(const Nan::FunctionCallbackInfo<v8::Value> &info)
{

    char* str = rust_hello();
    rust_capitalize(str);
    using namespace std;
    info.GetReturnValue().Set(Nan::New(str).ToLocalChecked());

    rust_free_string(str);
}

void Add(const Nan::FunctionCallbackInfo<v8::Value> &info)
{
    v8::Local<v8::Context> context = info.GetIsolate()->GetCurrentContext();

    if (info.Length() < 2)
    {
        Nan::ThrowTypeError("Wrong number of arguments");
        return;
    }

    if (!info[0]->IsNumber() || !info[1]->IsNumber())
    {
        Nan::ThrowTypeError("Wrong arguments");
        return;
    }

    double arg0 = info[0]->NumberValue(context).FromJust();
    double arg1 = info[1]->NumberValue(context).FromJust();
    v8::Local<v8::Number> num = Nan::New(rust_add(arg0, arg1));

    info.GetReturnValue().Set(num);
}

void ParseXlogToFile(const Nan::FunctionCallbackInfo<v8::Value> &info)
{
    v8::Local<v8::Context> context = info.GetIsolate()->GetCurrentContext();

    if (info.Length() < 2)
    {
        Nan::ThrowTypeError("Wrong number of arguments");
        return;
    }

    if (!info[0]->IsString() || !info[1]->IsString())
    {
        Nan::ThrowTypeError("Wrong arguments");
        return;
    }

    v8::String::Utf8Value src(info[0]->ToString());
    v8::String::Utf8Value dst(info[1]->ToString());

    auto debug_info_raw = awwleegay_parse_xlog_to_file(*src,*dst);
    auto debug_info = Nan::New<v8::String>(debug_info_raw).ToLocalChecked();
    rust_free_string(debug_info_raw);

    info.GetReturnValue().Set(debug_info);
}

void ParseXlogToFileTmp(const Nan::FunctionCallbackInfo<v8::Value> &info)
{
    v8::Local<v8::Context> context = info.GetIsolate()->GetCurrentContext();

    if (info.Length() != 1)
    {
        Nan::ThrowTypeError("Wrong number of arguments");
        return;
    }

    if (!info[0]->IsString())
    {
        Nan::ThrowTypeError("Wrong arguments");
        return;
    }

    v8::String::Utf8Value src(info[0]->ToString());
    
    std::time_t t = std::time(NULL);

    auto debug_info_raw = awwleegay_parse_xlog_to_file_tmp(*src); // FIXME: should pass ptr from c++ to rust
    char* i [2] = {debug_info_raw[0],debug_info_raw[1]};

    auto v8Array = Nan::New<v8::Array>();

    Nan::Set(v8Array, 0, Nan::New<v8::String>(i[0]).ToLocalChecked());
    Nan::Set(v8Array, 1, Nan::New<v8::String>(i[1]).ToLocalChecked());


    char mbstr[100];
    std::strftime(mbstr, sizeof(mbstr), "%H:%M:%S%.3f", std::localtime(&t));
    Nan::Set(v8Array, 2, Nan::New<v8::String>(mbstr).ToLocalChecked());

    info.GetReturnValue().Set(v8Array);
}


void ParseXlogToString(const Nan::FunctionCallbackInfo<v8::Value> &info)
{
    v8::Local<v8::Context> context = info.GetIsolate()->GetCurrentContext();

    if (info.Length() != 1)
    {
        Nan::ThrowTypeError("Wrong number of arguments");
        return;
    }

    if (!info[0]->IsString())
    {
        Nan::ThrowTypeError("Wrong arguments");
        return;
    }

    v8::String::Utf8Value src(info[0]->ToString());

    auto log_raw = awwleegay_parse_xlog_to_string(*src);
    auto log = Nan::New<v8::String>(log_raw).ToLocalChecked();
    rust_free_string(log_raw);

    info.GetReturnValue().Set(log);
}

#define export_native_to_node(native_method, node_method) \
    (void)exports->Set(                                   \
        context,                                          \
        Nan::New(node_method).ToLocalChecked(),           \
        Nan::New<v8::FunctionTemplate>(native_method)->GetFunction(context).ToLocalChecked());

void Init(v8::Local<v8::Object> exports)
{
    v8::Local<v8::Context> context = exports->CreationContext();

    export_native_to_node(Hello, "hello");
    export_native_to_node(Add, "add");
    export_native_to_node(ParseXlogToFile,    "parse_xlog_to_file");
    export_native_to_node(ParseXlogToFileTmp, "parse_xlog_to_file_tmp");
    export_native_to_node(ParseXlogToString,  "parse_xlog_to_string");
}

NODE_MODULE(hello, Init)
