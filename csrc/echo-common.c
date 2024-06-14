/*
 * This file was generated by orbit-idl-2 - DO NOT EDIT!
 */

#include <string.h>
#define ORBIT2_STUBS_API
#define ORBIT_IDL_C_COMMON
#define echo_COMMON
#include "echo.h"

static const CORBA_unsigned_long ORBit_zero_int = 0;

#ifndef ORBIT_IDL_C_IMODULE_echo
void _ORBIT_skel_small_Echo_echoString(POA_Echo             *_o_servant, gpointer            _o_retval,gpointer           *_o_args,CORBA_Context       _o_ctx,CORBA_Environment  *_o_ev,
void (*_impl_echoString)(PortableServer_Servant _servant, const CORBA_char * input, CORBA_Environment *ev)) {
_impl_echoString (_o_servant, *(const CORBA_char * *)_o_args[0], _o_ev);
}

#endif
#if ( (TC_IMPL_TC_Echo_0 == 'e') \
&& (TC_IMPL_TC_Echo_1 == 'c') \
&& (TC_IMPL_TC_Echo_2 == 'h') \
&& (TC_IMPL_TC_Echo_3 == 'o') \
) && !defined(TC_DEF_TC_Echo)
#define TC_DEF_TC_Echo 1
#ifdef ORBIT_IDL_C_IMODULE_echo
static
#endif
ORBIT2_MAYBE_CONST struct CORBA_TypeCode_struct TC_Echo_struct = {
{&ORBit_TypeCode_epv, ORBIT_REFCOUNT_STATIC},
CORBA_tk_objref,
0,
0,
ORBIT_ALIGNOF_CORBA_POINTER,
0,
0
,
NULL,
CORBA_OBJECT_NIL,
(char *)"Echo",
(char *)"IDL:Echo:1.0",
NULL,
NULL,
-1,
0,
0, 0
};
#endif

#ifndef ORBIT_IDL_C_IMODULE_echo
CORBA_unsigned_long Echo__classid = 0;
#endif

/* Interface type data */

static ORBit_IArg Echo_echoString__arginfo [] = {
	{ TC_CORBA_string,  ORBit_I_ARG_IN , (char *)"input" }
};

#ifdef ORBIT_IDL_C_IMODULE_echo
static
#endif
ORBit_IMethod Echo__imethods [] = {
	{
		{ 1, 1, Echo_echoString__arginfo, FALSE },
		{ 0, 0, NULL, FALSE },
		{ 0, 0, NULL, FALSE },
TC_void, (char *)"echoString", 10,
		0
}
};

static CORBA_string Echo__base_itypes[] = {
(char *)"IDL:omg.org/CORBA/Object:1.0"
};
#ifdef ORBIT_IDL_C_IMODULE_echo
static
#endif
ORBit_IInterface Echo__iinterface = {
TC_Echo,{1, 1, Echo__imethods, FALSE},
{1, 1, Echo__base_itypes, FALSE}
};

