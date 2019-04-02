use web_sys::Element;

pub fn init(toolbar: Element) {
    toolbar.set_inner_html("toolbar");
}
