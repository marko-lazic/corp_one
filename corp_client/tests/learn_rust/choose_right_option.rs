struct Data;

impl Data {
    pub fn crunch(&self) -> f32 {
        42.0
    }
}

struct Widget(Option<Data>);

impl Widget {
    fn data_a(&self) -> &Option<Data> {
        &self.0
    }

    fn data_b(&self) -> Option<&Data> {
        self.0.as_ref()
    }
}

#[test]
fn test_widget() {
    let widget = Widget(Some(Data));

    let a = widget.data_a();
    let b = widget.data_b();

    assert_eq!(a.is_some(), b.is_some());

    let crunch_a = a.as_ref().map(|data| data.crunch());

    let crunch_b = b.map(|data| data.crunch());

    assert_eq!(crunch_a, crunch_b);
}
