
#[derive(NodeLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct SawGroup;

fn saw_voice(mut commnads: Commands) {
    let saw_node = commands.spawn(SawNode::default()).id();

    // you could relate these nodes however you like!
    // but as children, they'll be easy to find
    commands.chain_node((AdsrNode::default(), ChildOf(saw_node)))
        .chain_node((SvfNode::default(), ChildOf(saw_node)))
        .connect(SawGroup);
}
