extern crate rustty;

pub mod core;
mod label;
mod dialog;
mod stdbutton;
mod checkbutton;
mod hlayout;
mod vlayout;
mod canvas;


pub use canvas::Canvas;
pub use dialog::Dialog;
pub use stdbutton::StdButton;
pub use checkbutton::CheckButton;
pub use hlayout::HorizontalLayout;
pub use vlayout::VerticalLayout;
pub use label::Label;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
