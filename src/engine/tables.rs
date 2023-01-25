//! A simple trait for interacting with various types of table used internally.

use std::{ops::{Index, IndexMut}, collections::HashMap, fmt::Display};

use super::{Symbol, SymbolType, LALRState, DFAState, CharacterSet, ProductionRule, LexicalGroup};

/// # Table
///
/// A simple trait used for accessing table-like objects.
///
/// This trait is used internally for the machine's constant table.  As long as
/// your table type implements this trait then you'll be cool.  Meaning you can
/// choose whatever language semantics you want with regards constants.
pub trait Table {
    /// The type for items stored and retrieved from the table.
    type Item;

    /// Add into an array-like list
    fn add(&mut self, item: Self::Item);

    /// Insert a value into the table using an index
    fn insert(&mut self, index: usize, value: Self::Item);

    /// Is the table empty or not?
    fn is_empty(&self) -> bool;

    /// Does the table contain the key or not?
    //fn contains_key(&self, name: String) -> bool;

    /// Retrieve a reference to a value stored in the table by key.
    fn get(&self, item: Self::Item) -> Option<&Self::Item>;

    /// Get number of entries
    fn len(&self) -> usize;

    // fn with_capacity(size: usize) -> 

    /// Resize to fixed size
    fn resize(&mut self, sz: usize);

    /// Clear all items and reset
    fn clear(&mut self);

}

#[derive(Debug,Default,Clone)]
/// SymbolTable
pub struct SymbolTable(Vec::<Symbol>);
//pub struct SymbolTable(HashMap<String, Symbol>);
impl SymbolTable {
    const DEFAULT: Symbol = Symbol { index: 0, name: String::new(), kind: SymbolType::Undefined };

    pub fn new() -> Self {
        SymbolTable(Vec::new())
    }
    /// could replace this with trait `Display`
    pub fn to_string(&self) -> String {
        format!("{}\n", self.0.iter().map(|s| s.to_string()).collect::<String>())
        // let mut buf = String::new();
        // self.0.as_slice()
        //     .into_iter()
        //     .map(|s| { 
        //         let str = format!("{} ",s);
        //         buf.push_str(str.as_str());
        //     }).next();
        // buf
    }
    pub fn as_handle(&self) -> String {
        format!("{}\n", self.0.iter().map(|s| s.as_handle() + " ").collect::<String>())
    }
    pub fn get(&self, name: String) -> Option<&Symbol> {
        for sym in &self.0 {
            if sym.name == name { return Some(sym) }
        }
        // for sym in self.0.as_slice() {
        //     if sym.name == name { return Some(&sym);}
        // }
        None
    }
    // gets 1st occurance of `SymbolType` in the table
    pub fn get_by_type(&self, kind: SymbolType) -> Option<&Symbol> {
        for sym in &self.0 {
            if sym.kind == kind { return Some(sym) }
        }
        None
    }
    pub fn with_capacity(size: usize) -> Self {
        SymbolTable(Vec::with_capacity(size))
    }
    pub fn push(&mut self, item: <SymbolTable as Table>::Item) {
        self.0.push(item);
    }

}
impl Table for SymbolTable {
    type Item = Symbol;

    fn clear(&mut self) {
        self.0.clear();
    }
    fn add(&mut self, item: Self::Item) {
        //self.0.insert(symbol.index, symbol); // this shifts elements
        let i = item.index;
        self.0[i] = item;
        //self.0.insert(index, item);
    }
    fn insert(&mut self, index: usize, value: Self::Item) {
        self.0[index] = value;
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn get(&self, item: Self::Item) -> Option<&Self::Item> {
        for sym in &self.0 {
            if *sym == item { return Some(sym) }
        }
        None
    }

    fn len(&self) -> usize {
        self.0.len()
    }
    fn resize(&mut self, sz: usize) {
        self.0.resize(sz, Self::DEFAULT);  //Symbol::default());
    }

    
}

impl From<Vec::<Symbol>> for SymbolTable {
    fn from(value: Vec::<Symbol>) -> Self {
        SymbolTable(value)
    }
}
impl Index<usize> for SymbolTable {
    type Output = Symbol;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
        //panic!("No symbol found at index {}",index)
    }

// impl IndexMut<usize> for SymbolTable {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.0[index]
//     }
// }
}

/// LRStateTable
pub struct LALRStateTable(Vec<LALRState>);
impl LALRStateTable {
    pub fn new() -> Self { LALRStateTable(Vec::new()) }

}
impl Table for LALRStateTable {
    type Item = LALRState;

    fn add(&mut self, state: Self::Item) {
        self.0.insert(state.index, state);
    }
    fn insert(&mut self, index: usize, value: Self::Item) {
        self.0[index] = value;
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn get(&self, item: Self::Item) -> Option<&Self::Item> {
        todo!()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn resize(&mut self, sz: usize) {
        self.0.resize_with(sz, || {Self::Item::default()})
    }
    fn clear(&mut self) {
        self.0.clear();
    }
}

impl Display for LALRStateTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.0.iter().map(|d| {format!("{}\n",d)}).collect::<String>())
    }
}

impl Index<usize> for LALRStateTable {
    type Output = LALRState;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for LALRStateTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// DFAStateTable
pub struct DFAStateTable(Vec<DFAState>);
impl DFAStateTable {
    pub fn new() -> Self { DFAStateTable(Vec::new()) }
    pub fn add(&mut self, state: DFAState) {
        let i = state.index;
        self.0.insert(i, state);
    }
}

impl Display for DFAStateTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}", self.0.iter().map(|d| {format!("{}\n",d)}).collect::<String>())
        //write!(f,"{}\n", self.0.iter().map(|s| s.to_string()).collect::<String>())
    }
}

impl Index<usize> for DFAStateTable {
    type Output = DFAState;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for DFAStateTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Table for DFAStateTable {
    type Item = DFAState;

    fn add(&mut self, state: Self::Item) {
        let i = state.index;
        self.0[i] = state;
    }
    fn insert(&mut self, index: usize, value: Self::Item) {
        self.0[index] = value;
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn get(&self, item: Self::Item) -> Option<&Self::Item> {
        todo!()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn resize(&mut self, sz: usize) {
        self.0.resize_with(sz, || {Self::Item::default()})
    }
    fn clear(&mut self) {
        self.0.clear();
    }
}

#[derive(Debug)]
/// CharacterSetTable
pub struct CharacterSetTable(Vec<CharacterSet>);
impl CharacterSetTable {
    pub fn new() -> Self { CharacterSetTable(Vec::new()) }
    pub fn add(&mut self, index: usize, chars: CharacterSet) {
        self.0[index] = chars;
        //self.0.insert(index, chars);
    }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn resize(&mut self, sz: usize) {
        self.0.resize(sz, CharacterSet::default())
    }
    pub fn contains(&self, charset: CharacterSet) -> bool {
        self.0.contains(&charset)
    }

}

impl Display for CharacterSetTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0.iter().map(|c| {c.to_string()}).collect::<String>())
    }
}

impl Index<usize> for CharacterSetTable {
    type Output = CharacterSet;
    /// charset_table[0]
    fn index(&self, index: usize) -> &CharacterSet {
        &self.0[index]
    }
}
impl IndexMut<usize> for CharacterSetTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// ProductionTable
pub struct ProductionTable(Vec<ProductionRule>);
impl ProductionTable {
    pub fn new() -> Self { ProductionTable(Vec::new()) }
    pub fn with_capacity(capacity: usize) -> Self {ProductionTable(Vec::with_capacity(capacity))}

    pub fn add(&mut self, rule: ProductionRule) {
        let index = rule.index;
        self.0[index] = rule;
    }
}

impl Display for ProductionTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}",self.0.iter().map(|c| {format!("{}",c.to_string())}).collect::<String>())
    }
}

impl Index<usize> for ProductionTable {
    type Output = ProductionRule;
    fn index(&self, index: usize) -> &ProductionRule {
        &self.0[index]
    }
}
impl IndexMut<usize> for ProductionTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Table for ProductionTable {
    type Item = ProductionRule;

    fn add(&mut self, item: Self::Item) {
        let i = item.index;
        self.0[i] = item;
    }

    fn insert(&mut self, index: usize, value: Self::Item) {
        self.0[index] = value;
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn get(&self, item: Self::Item) -> Option<&Self::Item> {
        if item.index <= self.len() {
            return Some(&self.0[item.index])
        }
        None
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn resize(&mut self, sz: usize) {
        self.0.resize_with(sz,  || ProductionRule::default());
    }

    fn clear(&mut self) {
        self.0.clear()
    }
}


/// GroupTable
pub struct GroupTable(Vec<LexicalGroup>);
impl GroupTable {
    pub fn new() -> Self { GroupTable(Vec::new()) }
    pub fn add(&mut self, group: LexicalGroup) {
        let i = group.index;
        self.0[i] = group;
    }

}
impl Index<usize> for GroupTable {
    type Output = LexicalGroup;
    fn index(&self, index: usize) -> &LexicalGroup {
        &self.0[index]
    }
}



#[cfg(test)]
mod test {

    #[test]
    fn display() {

    }
}


