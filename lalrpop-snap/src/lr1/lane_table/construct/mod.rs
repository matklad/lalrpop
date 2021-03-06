//!

use collections::{Map, Set};
use ena::unify::UnificationTable;
use grammar::repr::*;
use lr1::build;
use lr1::core::*;
use lr1::first::FirstSets;
use lr1::lookahead::{Lookahead, TokenSet};
use lr1::lane_table::lane::LaneTracer;
use lr1::lane_table::table::{ConflictIndex, LaneTable};
use lr1::lane_table::table::context_set::OverlappingLookahead;
use lr1::state_graph::StateGraph;
use std::rc::Rc;

mod merge;
use self::merge::Merge;

mod state_set;
use self::state_set::StateSet;

pub struct LaneTableConstruct<'grammar> {
    grammar: &'grammar Grammar,
    first_sets: FirstSets,
    start_nt: NonterminalString,
}

impl<'grammar> LaneTableConstruct<'grammar> {
    pub fn new(grammar: &'grammar Grammar, start_nt: NonterminalString) -> Self {
        let first_sets = FirstSets::new(grammar);
        LaneTableConstruct {
            grammar: grammar,
            start_nt: start_nt,
            first_sets: first_sets,
        }
    }

    pub fn construct(self) -> Result<Vec<LR1State<'grammar>>, LR1TableConstructionError<'grammar>> {
        let TableConstructionError {
            states,
            conflicts: _,
        } = {
            match build::build_lr0_states(self.grammar, self.start_nt) {
                // This is the easy (and very rare...) case.
                Ok(lr0) => return Ok(self.promote_lr0_states(lr0)),
                Err(err) => err,
            }
        };

        // Convert the LR(0) states into LR(0-1) states.
        let mut states = self.promote_lr0_states(states);

        // For each inconsistent state, apply the lane-table algorithm to
        // resolve it.
        for i in 0.. {
            if i >= states.len() {
                break;
            }

            match self.resolve_inconsistencies(&mut states, StateIndex(i)) {
                Ok(()) => {}
                Err(_) => {
                    // We failed because of irreconcilable conflicts
                    // somewhere. Just compute the conflicts from the final set of
                    // states.
                    let conflicts: Vec<Conflict<'grammar, TokenSet>> = states
                        .iter()
                        .flat_map(|s| Lookahead::conflicts(&s))
                        .collect();
                    return Err(TableConstructionError {
                        states: states,
                        conflicts: conflicts,
                    });
                }
            }
        }

        Ok(states)
    }

    /// Given a set of LR0 states, returns LR1 states where the lookahead
    /// is always `TokenSet::all()`. We refer to these states as LR(0-1)
    /// states in the README.
    fn promote_lr0_states(&self, lr0: Vec<LR0State<'grammar>>) -> Vec<LR1State<'grammar>> {
        let all = TokenSet::all();
        debug!("promote_lr0_states: all={:?}", all);
        lr0.into_iter()
            .map(|s| {
                let items = s.items
                    .vec
                    .iter()
                    .map(|item| Item {
                        production: item.production,
                        index: item.index,
                        lookahead: all.clone(),
                    })
                    .collect();
                let reductions = s.reductions
                    .into_iter()
                    .map(|(_, p)| (all.clone(), p))
                    .collect();
                State {
                    index: s.index,
                    items: Items {
                        vec: Rc::new(items),
                    },
                    shifts: s.shifts,
                    reductions: reductions,
                    gotos: s.gotos,
                }
            })
            .collect()
    }

    fn resolve_inconsistencies(
        &self,
        states: &mut Vec<LR1State<'grammar>>,
        inconsistent_state: StateIndex,
    ) -> Result<(), StateIndex> {
        debug!(
            "resolve_inconsistencies(inconsistent_state={:?}/{:#?}",
            inconsistent_state, states[inconsistent_state.0]
        );

        let actions = super::conflicting_actions(&states[inconsistent_state.0]);
        if actions.is_empty() {
            return Ok(());
        }

        debug!("resolve_inconsistencies: conflicting_actions={:?}", actions);

        let table = self.build_lane_table(states, inconsistent_state, &actions);

        // Consider first the "LALR" case, where the lookaheads for each
        // action are completely disjoint.
        if self.attempt_lalr(&mut states[inconsistent_state.0], &table, &actions) {
            return Ok(());
        }

        // Construct the initial states; each state will map to a
        // context-set derived from its row in the lane-table. This is
        // fallible, because a state may be internally inconstent.
        //
        // (To handle unification, we also map each state to a
        // `StateSet` that is its entry in the `ena` table.)
        let rows = table.rows()?;
        let mut unify = UnificationTable::<StateSet>::new();
        let mut state_sets = Map::new();
        for (&state_index, context_set) in &rows {
            let state_set = unify.new_key(context_set.clone());
            state_sets.insert(state_index, state_set);
            debug!(
                "resolve_inconsistencies: state_index={:?}, state_set={:?}",
                state_index, state_set
            );
        }

        // Now merge state-sets, cloning states where needed.
        let mut merge = Merge::new(
            &table,
            &mut unify,
            states,
            &mut state_sets,
            inconsistent_state,
        );
        let beachhead_states = table.beachhead_states();
        for beachhead_state in beachhead_states {
            match merge.start(beachhead_state) {
                Ok(()) => {}
                Err((source, _)) => return Err(source),
            }
        }
        merge.patch_target_starts(&actions);

        Ok(())
    }

    fn attempt_lalr(
        &self,
        state: &mut LR1State<'grammar>,
        table: &LaneTable<'grammar>,
        actions: &Set<Action<'grammar>>,
    ) -> bool {
        match table.columns() {
            Ok(columns) => {
                debug!("attempt_lalr, columns={:#?}", columns);
                columns.apply(state, actions);
                true
            }
            Err(OverlappingLookahead) => {
                debug!("attempt_lalr, OverlappingLookahead");
                false
            }
        }
    }

    fn build_lane_table(
        &self,
        states: &[LR1State<'grammar>],
        inconsistent_state: StateIndex,
        actions: &Set<Action<'grammar>>,
    ) -> LaneTable<'grammar> {
        let state_graph = StateGraph::new(states);
        let mut tracer = LaneTracer::new(
            self.grammar,
            self.start_nt,
            states,
            &self.first_sets,
            &state_graph,
            actions.len(),
        );
        for (i, &action) in actions.iter().enumerate() {
            tracer.start_trace(inconsistent_state, ConflictIndex::new(i), action);
        }
        tracer.into_table()
    }
}
