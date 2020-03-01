#ifndef PLATEFORM_H_INCLUDED
#define PLATEFORM_H_INCLUDED

#ifdef __APPLE__
/* pass */

#elif _WIN32
#pragma comment(lib,"ws2_32.lib")
#pragma comment(lib, "userenv.lib")

extern "C"  int __CRTDECL __ms_vsnprintf(
	_Out_writes_opt_(_BufferCount) _Always_(_Post_z_) char* const _Buffer,
	_In_ size_t const _BufferCount,
	_In_z_ _Printf_format_string_ char const* const _Format,
	va_list _ArgList)
{
	vsnprintf(_Buffer, _BufferCount, _Format, _ArgList);
	return(0);
}

#endif


#endif /* PLATEFORM_H_INCLUDED */
