/*
 * This file was generated by orbit-idl-2 - DO NOT EDIT!
 */

#include <string.h>
#define ORBIT2_STUBS_API
#include "echo.h"

void Echo_echoString(Echo _obj, const CORBA_char * input, CORBA_Environment *ev){
gpointer _args[1];
_args[0] = (gpointer)&input;
ORBit_c_stub_invoke (_obj, &Echo__iinterface.methods, 0, NULL, _args, NULL, ev, Echo__classid, G_STRUCT_OFFSET (POA_Echo__epv, echoString),
(ORBitSmallSkeleton) _ORBIT_skel_small_Echo_echoString);

}
