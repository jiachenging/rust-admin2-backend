#[macro_export]
macro_rules! render_layout {
    ($path: expr) => ({
        include!(concat!("../templates/layouts", $path))
    })
}

#[macro_export]
macro_rules! render_view {
    ($path: expr) => ({
        include!(concat!("../../templates/pages", $path))
    })
}