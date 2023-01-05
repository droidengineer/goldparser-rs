//! Initial States, DFA States, LALR States
//! 
//! 
//! http://goldparser.org/doc/egt/record-initial-states.htm



/// The `InitialStateRecord` only occurs once in the `EnhancedGrammarTable` file. 
/// It will contain the initial states for both the DFA and LALR algorithms.  
/// The record is preceded by a byte field contains the value 73, the ASCII code for the letter 'I'.
pub struct InitialStatesRecord {
    /// The initial state in the Deterministic Finite Automata table. Normally, due to how the generation algorithm is implemented, this value should be 0
    pub dfa: u16,
    /// The initial state in the LALR state table. Like the DFA state table, this value should normally be 0
    pub lalr: u16,
}

impl InitialStatesRecord {
    pub fn new(dfa: u16, lalr: u16) -> Self {
        InitialStatesRecord { dfa, lalr } 
    }
}
impl std::fmt::Display for InitialStatesRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = format!("Initial States: DFA({}) LALR({})",self.dfa, self.lalr);
        write!(f,"{}", disp)
    }
}

/// Each record describing a state in the `DFAStateTable` is preceded by a byte field containing the value 68 
/// - the ASCII code for "D". The file will contain one of these records for each state in the table. The 
/// `TableCountsRecord`, which precedes any `DFAStateRecord`, will contain the total number of states.
/// The record contains information about the state itself: where it is located in the DFA State table and 
/// what symbols can be accepted (if any). Following this, there is a series of fields which describe each 
/// edge of the states. A DFA state can contain 0 or more edges, or links, to other states in the Table. 
/// These are organized in groups of 3 and will constitute the rest of the record.
/// http://goldparser.org/doc/egt/record-dfa-state.htm
pub struct DFAStateRecord {
    /// This parameter holds the index of the DFA state in the DFA State Table
    pub index: u16,
    /// Each DFA state can accept one of the grammar's terminal symbols. If the state accepts a terminal symbol, the value will be set to True and the Accept Index parameter will contain the symbol's index
    pub accept: u8,
    /// If the state accepts a terminal symbol, this field will contain the symbol's index in the Symbol Table. Otherwise, the value in this field should be ignored
    pub accept_idx: u16,
    /// This field is reserved for future use
    pub reserved: u8,
    /// 
    pub edges: Vec<DFAEdge>,
}
impl DFAStateRecord {
    const CODE: u8 = 68; //'D';
}

/// * Each edge contains a series of characters that are used to determine whether the Deterministic Finite Automata 
/// will follow it. The actual set of valid characters is not stored in this field, but, rather, an index in the 
/// `CharacterSetTable`.
/// * Target Index 	Integer 	Each edge is linked to state in the DFA Table. This field contains the index of that state.
/// * (Reserved) 	Empty 	This field is reserved for future use.
pub struct DFAEdge(u16,u16,u8);

/// Each record describing a state in the LALR State Table is preceded by a byte field containing the value 76 
/// - the ASCII code for "L". The file will contain one of these records for each state in the table. The 
/// `TableCountsRecord`, which precedes any LALR records, will contain the total number of states.
///
/// A LALR State contains a series of actions that are performed based on the next token. The record mostly 
/// consists of a series of fields (in groups of 4) which describe each of these actions.
/// http://goldparser.org/doc/egt/record-lalr-state.htm
pub struct LALRSateRecord {
    /// This parameter holds the index of the state in the `LALRStateTable`
    pub index: u16,
    /// Reserved for future use
    pub reserved: u8,
    /// 1 or more `LALRAction`s
    pub actions: Vec<LALRAction>,
}
impl LALRSateRecord {
    const CODE: u8 = 68; // 'D'
}

pub struct LALRAction {
    /// Contains the index in the `SymbolTable`
    pub index: u16,
    /// This field contains a value that represents the action that LALR parsing engine is to take based on the symbol. These values are enumerated below
    pub action: ActionType,
    /// Depending on the value of the Action field, the target will hold different types of information
    pub target: u16,
    /// Reserved for future use
    pub reserved: u8,
}

pub enum ActionType {
    /// This action indicates the symbol is to be shifted. The Target field will contain the index of the state in the LALR State table that the parsing engine will advance to.
    Shift = 1,
    /// This action denotes that the parser can reduce a rule. The Target field will contain the index of the rule in the `RuleTable`.
    Reduce = 2,
    /// This action is used when a rule is reduced and the parser jumps to the state that represents the shifted nonterminal. The Target field will contain the index of the state in the `LALRStateTable` that the parsing engine will jump to after a reduction if completed.
    Goto = 3,
    /// When the parser encounters the `Accept` action for a given symbol, the source text is accepted as correct and complete. In this case, the Target field is not needed and should be ignored.
    Accept = 4,
}