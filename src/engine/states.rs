//! Initial States, DFA States, LALR States
//! 
//! 
//! http://goldparser.org/doc/egt/record-initial-states.htm

use std::fmt::Display;

use super::{Symbol, CharacterSet,};

#[derive(Debug,Default,Clone,PartialEq)]
/// *Represents a state in the Deterministic Finite Automaton which is used by the Tokenizer.*
/// 
/// Each record describing a state in the `DFAStateTable` is preceded by a byte field containing the value 68, 
/// the ASCII code for "D". The file will contain one of these records for each state in the table. The 
/// `TableCountsRecord`, which precedes any `DFAState`, will contain the total number of states.
/// The record contains information about the state itself: where it is located in the DFA State table and 
/// what symbols can be accepted (if any). Following this, there is a series of fields which describe each 
/// edge of the states. A DFA state can contain 0 or more edges, or links, to other states in the Table. 
/// These are organized in groups of 3 and will constitute the rest of the record.
/// http://goldparser.org/doc/egt/record-dfa-state.htm
pub struct DFAState {
    /// This parameter holds the index of the DFA state in the `DFAStateTable`
    /// DEPRECATED
    pub index: u16,
    /// Each `DFAState` can accept one of the grammar's terminal symbols. If the state accepts a 
    /// terminal symbol, the value will be set to True and the `accept_idx` parameter will contain 
    /// the symbol's index
    pub accept: bool,
    /// If the state accepts a terminal symbol, this field will contain the symbol's index in the 
    /// `SymbolTable`. Otherwise, the value in this field should be ignored
    //pub accept_index: u16,
    pub accept_symbol: Symbol,
    /// See `DFAEdge`
    pub edges: Vec<DFAEdge>,
}
impl DFAState {
    pub fn new(index: u16, accept: bool, accept_symbol: Symbol, edges: Vec<DFAEdge>) -> Self {
        // The edges still contain indexes for targets
        DFAState { index, accept, accept_symbol, edges } 
    }
    pub fn find_edge(&self, ch: u16) -> Option<u16> {
        for edge in &self.edges {
            if edge.chars.contains(ch) {
                return Some(edge.target_state);
            }
        }
        None
    }
    // pub fn accept_symbol(&self) -> &Symbol {

    // }
}
impl Display for DFAState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[DFA State {:3}] ", self.index)?;
        if self.accept { write!(f, "Terminal: \'{}\' ", self.accept_symbol.name)?;}
        writeln!(f,"Edges:")?;
        write!(f,"{}",self.edges.iter().map(|e| {e.to_string() }).collect::<String>())
        // let disp = format!("@{:04X} accept: {} terminal: {} Edges: {:?}",
        //     self.index, self.accept, self.accept_symbol, self.edges);
        // writeln!(f,"{}", disp)
    }
}

#[derive(Debug,Clone,PartialEq)]
/// *Used to represent an virticese edge*
/// 
/// Each state in the **DFA** contains multiple edges which link to other states in the automata
/// * `index` is index into `CharacterSetTable`
/// * `target_state_idx` 
pub struct DFAEdge {
    /// Each edge contains a series of characters that are used to determine whether the DFA will follow it. 
    /// The actual set of valid characters is not stored in this field, but, rather, an index in the 
    /// `CharacterSetTable`
    pub chars: CharacterSet,
    /// Each edge is linked to state in the DFA Table. This field contains the index of that state
    pub target_state: u16, //DFAState,
}
impl DFAEdge {
}
impl Display for DFAEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Goto {:4} Character {}",self.target_state,self.chars)
    }
}

//---------------------------[LALRState]

#[derive(Debug,Default,Clone,PartialEq)]
/// Each record describing a state in the LALR State Table is preceded by a byte field containing the value 76, 
/// the ASCII code for "L". The file will contain one of these records for each state in the table. The 
/// `TableCountsRecord`, which precedes any LALR records, will contain the total number of states.
///
/// A LALR State contains a series of actions that are performed based on the next Token. The record mostly 
/// consists of a series of fields (in groups of 4) which describe each of these actions.
/// http://goldparser.org/doc/egt/record-lalr-state.htm
pub struct LALRState
 {
    /// This parameter holds the index of the state in the `LALRStateTable`
    /// DEPRECATED
    pub index: u16,
    /// 1 or more `LALRAction`s
    pub actions: Vec<LALRAction>,
}
impl LALRState
 {
    pub fn new(index: u16, actions: Vec<LALRAction>) -> Self {
        LALRState { index, actions } 
    }

    pub fn find_action(&self, symbol: &Symbol) -> Option<&LALRAction> {
        for action in &self.actions {
            if action.symbol == *symbol {
                return Some(action)
            }
        }
        None
    }
    pub fn clear(&mut self) { self.actions.clear(); }
    
}

impl Display for LALRState
 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"[LALR State {:3}] ", self.index)?;
        
        let actions = self.actions.as_slice();
        // let str : Vec<String> = actions.into_iter()
        //     .map(|a| format!("{}\n",a))
        //     .collect();
        writeln!(f,"{}",actions.iter()
                               .map(|a| a.to_string())//format!("{}\n",a))
                               .collect::<String>())
        // let disp = format!("@{:04X} LALRState[{}] Actions: {:?}",self.index, self.index, str.iter());
        // write!(f,"{}", disp)
    }
}


#[derive(Debug,Clone,PartialEq)]
pub struct LALRAction {
    /// Contains the index in the `SymbolTable`
    /// Optionally could store copy directly here as `entry: Symbol` OR &ref
    pub symbol: Symbol,
    /// This field contains a value that represents the action that LALR parsing engine is 
    /// to take based on the symbol. These values are enumerated below
    pub action: ActionType,
    /// Depending on the value of the Action field, the target will hold different types 
    /// of information: action{Shift,Goto} `LALRStateTable[target]`
    /// action{Reduce} `RuleTable[target]`
    pub target_idx: u16,
}

impl LALRAction {
    pub fn new(symbol: Symbol, action: ActionType, target_idx: u16) -> Self {
        LALRAction { symbol, action, target_idx }
    }
    pub fn index(&self) -> usize { self.target_idx as usize}
    // pub fn target(&self) -> Option<impl Table> {
    //     match self.action {
    //         ActionType::Shift |
    //         ActionType::Goto => lalrstatetable[self.target_idx],
    //     }
    // }
    
}
impl Display for LALRAction
 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:16} ",self.symbol.as_bnf())?;
        match self.action {
            ActionType::Goto => writeln!(f,"Goto to LALR State {}",self.target_idx),
            ActionType::Shift => writeln!(f,"Shift to LALR State {}",self.target_idx),
            ActionType::Reduce => writeln!(f,"Reduce to Rule {}",self.target_idx),
            _ => writeln!(f,"LALRAction ERROR"),
        }

    }
}

enum_from_primitive! {
    #[derive(Debug,Default,Copy,Clone,PartialEq)]
    pub enum ActionType {
        #[default]
        Undefined,
        /// This action indicates the symbol is to be shifted. 
        /// The Target field will contain the index of the state in the `LALRStateTable` that
        /// the parsing engine will advance to.
        Shift = 1,
        /// This action denotes that the parser can reduce a rule. 
        /// The Target field will contain the index of the rule in the `RuleTable`.
        Reduce = 2,
        /// This action is used when a rule is reduced and the parser jumps to the state
        ///  that represents the shifted nonterminal. The Target field will contain the 
        /// index of the state in the `LALRStateTable` that the parsing engine will jump 
        /// to after a reduction if completed.
        Goto = 3,
        /// When the parser encounters the `Accept` action for a given symbol, the source 
        /// text is accepted as correct and complete. In this case, the Target field is not 
        /// needed and should be ignored.
        Accept = 4,
    }
}





////////////////////////////////[ TESTING ]
#[cfg(test)]
mod test {


}