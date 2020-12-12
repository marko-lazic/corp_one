mod control;
mod input_plugin;

pub use input_plugin::input::InputPlugin;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
