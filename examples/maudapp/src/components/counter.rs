use quasar::*;

struct CounterData {
    count: u32,
}

impl Renderable for CounterData {
    fn render(&self, _node: &Node, _app: &AppContext) -> String {
        (html! {
            p { "Count: " (self.count) }
            button { "+1" }
        }).into_string()
    }
}

pub fn init(app: &QuasarApp) {
    let component = CounterData {
        count: 0
    };

    app.bind("#counter", component)
        .on_each(EventType::Click, "button", |mut evt| {
            evt.binding.data_mut().count += 1;
        });
}