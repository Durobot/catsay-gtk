use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::env;

fn main()
{
    let app = Application::new(
        Some("com.shinglyu.catsay-gui"),
        gio::ApplicationFlags::empty()
    ).expect("Failed to initialize GTK (gtk::Application::new() failed)");
    app.connect_startup(|app|
    {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(350, 70);
        window.connect_delete_event(|_win, _| // warning: unused variable: `win` help: if this is intentional, prefix it with an underscore: `_win`
        {
            //win.destroy(); -- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
            //gtk::main_quit(); -- thread 'main' panicked at 'Attempted to quit a GTK main loop when none is running.'
            Inhibit(false) // Don't prevent the default behavior (i.e. close)
        });
        window.show_all();
    });
    app.connect_activate(|_| {});
    app.run(&env::args().collect::<Vec<_>>()); // start the event loop (? - see the comment on gtk::main_quit() above)
}
