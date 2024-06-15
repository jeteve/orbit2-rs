use orbit2_sys::echo::{
    self,
    //servant::{self, CORBA_Object},
};

fn main() {
    println!("Hello, I am a server");
    echo::servant::init_global_structs();

    //let mut servant: CORBA_Object = null_mut();
}
