use orbtk::prelude::*;

use crate::MainState;

widget!(
    MainView<MainState> {
        title: String
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(TextBlock::new().text(("title", id)).build(ctx))
    }
}
