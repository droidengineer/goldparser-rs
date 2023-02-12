Span for rule and code for token ala pest
☐ Add ability to unwind stack
☐ Add rule handler map (rule: &str, R: RuleHandler) that maps a "rule" to the
  function/etc that handles it
Cursor Position of &str w/ useful string parsing methods
Refactor engine::Reduction out and into parser::Reduction for use in template
  bring in 'static lifetime
Refactor engine::egt and engine::builder into engine::parser
☐ Instead of panic!ing on EOF in SourceReader (source.rs:33:29), return an Option<ch>


  ✅ ☐ 🗹



