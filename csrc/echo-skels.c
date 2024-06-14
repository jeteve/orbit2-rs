/*
 * This file was generated by orbit-idl-2 - DO NOT EDIT!
 */

#include <string.h>
#define ORBIT2_STUBS_API
#include "echo.h"

static ORBitSmallSkeleton get_skel_small_Echo(POA_Echo *servant,
const char *opname,gpointer *m_data, gpointer *impl)
{
switch(opname[0]) {
case 'e':
if(strcmp((opname + 1), "choString")) break;
*impl = (gpointer)servant->vepv->Echo_epv->echoString;
{ORBit_IInterface *volatile _t_=&Echo__iinterface;*m_data = (gpointer)&_t_->methods._buffer [0];}
return (ORBitSmallSkeleton)_ORBIT_skel_small_Echo_echoString;
break;
default: break; 
}
return NULL;
}

void POA_Echo__init(PortableServer_Servant servant,
CORBA_Environment *env)
{
  static PortableServer_ClassInfo class_info = {NULL, (ORBit_small_impl_finder)&get_skel_small_Echo, "IDL:Echo:1.0", &Echo__classid, NULL, &Echo__iinterface};
  PortableServer_ServantBase__init (       ((PortableServer_ServantBase *)servant), env);
   ORBit_skel_class_register (&class_info,
   (PortableServer_ServantBase *)servant, POA_Echo__fini,
   ORBIT_VEPV_OFFSET (POA_Echo__vepv, Echo_epv),
   (CORBA_unsigned_long) 0);}

void POA_Echo__fini(PortableServer_Servant servant,
CORBA_Environment *env)
{
  PortableServer_ServantBase__fini(servant, env);
}

