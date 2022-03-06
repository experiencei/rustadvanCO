bin --> app.rs

        lib -->     Codec
        |              Audio
        |              Video
        |      
        transcode.rs
        ""mod.rs""


# in  ""mod.rs"" we have 
      pub mod codec
      pub mod transcode

      mod.rs must always be present*

pub keyword make a fn available outside it boundaries

//ACCESSING FUNCTIONALITY
    use super::mp3

    pub fn sample() {
        mp3::some_fn();
        super::mp3::some_fn();
        crate::codec::audio::mp3::some_fn();
        super::super::video::h264::some_fn();
    }

//MODULE ALIASES
 pub fn sample() {
     use crate::transcode as tc;
     tc::some_fn();
 }

 // RE-EXPORTING MODULES
     pub use audio::mp3;
           // making the it avilable in codec module
     ___________________

     use crate::codec::mp3;
