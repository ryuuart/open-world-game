use taffy::prelude::*;

mod draw;

fn taffy_test() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let root = build_tree(&mut tree).expect("Couldn't build tree");

    tree.compute_layout(root, Size::MAX_CONTENT)
        .expect("Couldn't compute layout");
    tree.print_tree(root);

    draw::draw_tree(tree.layout(root).expect("Couldn't get the layout"));
}

fn build_tree(tree: &mut TaffyTree<()>) -> Result<NodeId, taffy::TaffyError> {
    let container = tree.new_leaf(Style {
        size: Size {
            width: length(800.0),
            height: length(600.0),
        },
        padding: Rect {
            left: LengthPercentage::Length(16.0),
            right: LengthPercentage::Length(16.0),
            top: LengthPercentage::Length(32.0),
            bottom: LengthPercentage::Length(8.0),
        },
        ..Default::default()
    })?;

    Ok(container)
}

fn main() {
    taffy_test();
}
