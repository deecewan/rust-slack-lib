enum FirstItem {
    Thing(Thing),
}
enum SecondItem {
    Thing(Thing),
}

#[impl_for::impl_for(FirstItem, SecondItem)]
struct Thing {}

#[test]
fn works() {
    FirstItem::from(Thing {});
    SecondItem::from(Thing {});
}
