// use std::fs;

use orbit2::*;
use orbit2_sample_idls::echo::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let orb = CorbaORB::new("orbit-local-orb", &[])?;

    let ref_data = "IOR:010000000d00000049444c3a4563686f3a312e3000000000030000000054424f540000000101020005000000554e4958000000000a0000006c6f63616c686f73740000002b0000002f746d702f6f726269742d7673636f64652f6c696e632d3538392d302d36356464636565363235376633000000000000caaedfba58000000010102002b0000002f746d702f6f726269742d7673636f64652f6c696e632d3538392d302d363564646365653632353766330000000000001c000000000000004ee6b410d1cde8a8c02b28282828282801000000eb03d77f01000000480000000100000002000000050000001c000000000000004ee6b410d1cde8a8c02b28282828282801000000eb03d77f01000000140000000100000001000105000000000901010000000000";
    //let str = fs::read_to_string("echo.ref")?;

    let mut echo_obj = orb.import_object::<Echo, _>(ref_data)?;

    echo_obj.with(|obj, ev| {
        unsafe {
            Echo_echoString(
                obj.as_corba_object(),
                CorbaCharPtr::new("Pizza")?.into(),
                ev.as_corba_environment_ptr(),
            )
        };
        Ok(())
    })?;

    Ok(())
}
