//! tables
//! Provides standard expected functionality for the tables used by the parser.

use std::{fmt::Display, ops::{Index,IndexMut}};

use super::Symbol;


pub trait TableItem {
    type Item;

    fn get(&self, item: Self::Item) -> Option<&Self::Item>;

}
// impl<T:Default+Display+Clone> TableItem for T {
//     type Item = T;
// }

#[derive(Default)]
pub struct GPTable<T>
where 
    T: Default+Clone+Display,
{
    table: Vec<T>,

}
impl<T> GPTable<T>
where 
    T: Default+Clone+Display+PartialEq<T>,
    
{
    pub fn new_sized(size: usize) -> Self {
        Self {
            table: vec![T::default();size]
        }
    }
    pub fn len(&self) -> usize {self.table.len()}
    pub fn is_empty(&self) -> bool {self.table.is_empty()}
    pub fn resize(&mut self, sz: usize) {self.table.resize(sz, T::default());}
    pub fn get(&self, by: T) -> Option<&T> {
        for s in &self.table {
            if *s == by { return Some(s) }
        }
        None

    }
}

impl<T:Default+Clone+Display> Index<usize> for GPTable<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}
impl<T:Default+Clone+Display> IndexMut<usize> for GPTable<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.table[index]
    }
}
impl<T:Default+Clone+Display> Display for GPTable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.table.iter().map(|t| t.to_string()).collect::<String>())
    }
}

// pub fn get_sym_by_name(symtab: &GPTable<Symbol>, sym: Symbol) -> Option<&Symbol> {

// }