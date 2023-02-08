// #![feature(test)]
// extern crate test;
#[macro_use] extern crate log;



#[macro_use] extern crate enum_primitive;
extern crate num_traits;


pub mod engine;
pub mod parser;

pub use engine::Parser;
pub use parser::GOLDParser;


pub mod test {
    pub const GP_TEST_EGT: &str = r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\.ref\goldparser-test-new.egt";
    pub const GP_TEST_SRC: &str = r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\.ref\goldparser-test.asm";
    pub const GP_SIMPLE_EGT: &str = r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\examples\simple.egt";
    pub const GP_SIMPLE_SRC: &str = r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\examples\simple.src";
    pub const GP_TINY_EGT: &str = r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\examples\tiny.egt";
    pub const GP_TINY_SRC: &str = r"D:\Users\Gian\prog\repos\RUST\goldparser-rs\examples\tiny.src";
    
    pub fn init_logger() {
        let _ = env_logger::builder()
            // include all events in tests
            .filter_level(log::LevelFilter::max())
            // ensure events are captured by `cargo test`
            .is_test(true)
            // ignore errors initializing the logger if tests race to configure it
            .try_init();
    }
}
