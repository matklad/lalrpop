use collections::Set;
use lr1::core::*;
use lr1::lookahead::Lookahead;
use grammar::repr::*;

mod construct;
mod lane;
mod table;

#[cfg(test)]
mod test;

pub fn build_lane_table_states<'grammar>(
    grammar: &'grammar Grammar,
    start: NonterminalString,
) -> LR1Result<'grammar> {
    construct::LaneTableConstruct::new(grammar, start).construct()
}

fn conflicting_actions<'grammar, L: Lookahead>(
    state: &State<'grammar, L>,
) -> Set<Action<'grammar>> {
    let conflicts = L::conflicts(state);
    let reductions = conflicts.iter().map(|c| Action::Reduce(c.production));
    let actions = conflicts.iter().map(|c| c.action);
    reductions.chain(actions).collect()
}
