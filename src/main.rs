fn main() {
    loop {
        utils::oinfo!("info");
        utils::onote!("note");
        utils::owarning!("warning");
        utils::oerror!("error");
        utils::ofatal!("fatal")
    }
}