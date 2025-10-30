pub trait HtmlTag {
    fn name() -> &'static str;
    fn is_void() -> bool;
}

pub struct HtmlDivTag;
impl HtmlTag for HtmlDivTag {
    fn name() -> &'static str {
        "div"
    }
    fn is_void() -> bool {
        false
    }
}
